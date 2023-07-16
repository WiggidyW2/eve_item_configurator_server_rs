use super::super::{
    accessors::Accessor,
    error::Error,
    pb::{self, weve_esi::WeveEsi},
};
use super::service::Service;

impl<A: Accessor> Service<A> {
    pub fn list_bb_contracts_unauthorized(
        &self,
        new_token: String,
    ) -> pb::item_configurator::BuybackContractsRep {
        pb::item_configurator::BuybackContractsRep {
            contracts: Vec::new(),
            refresh_token: new_token,
            authorized: false,
        }
    }

    async fn bb_check(&self, hash_code: String, language: &str) -> Result<pb::buyback::Rep, Error> {
        Ok(self
            .buyback_client
            .clone()
            .check(pb::buyback::CheckReq {
                hash: hash_code,
                language: language.to_string(),
            })
            .await
            .map_err(|e| Error::BuybackCheck(e))?
            .into_inner())
    }

    async fn bb_check_then_buy(
        &self,
        hash_code: String,
        language: &str,
    ) -> Result<(pb::buyback::Rep, pb::buyback::Rep), Error> {
        let check_rep = self.bb_check(hash_code, language).await?;

        let buy_rep = self
            .buyback_client
            .clone()
            .buy(pb::buyback::BuyReq {
                location: check_rep.location.clone(),
                language: language.to_string(),
                items: check_rep
                    .items
                    .iter()
                    .filter_map(|i| match i.parent_type_id == i.type_id {
                        true => Some(pb::buyback::ReqItem {
                            type_id: i.type_id,
                            quantity: i.quantity as u32,
                        }),
                        false => None,
                    })
                    .collect(),
            })
            .await
            .map_err(|e| Error::BuybackBuy(e))?
            .into_inner();

        Ok((check_rep, buy_rep))
    }

    pub async fn list_bb_contracts_authorized(
        &self,
        req: pb::item_configurator::BuybackContractsReq,
        new_token: String,
    ) -> Result<pb::item_configurator::BuybackContractsRep, Error> {
        let esi_contracts = self
            .weve_esi_client
            .exchange_contracts(
                pb::weve_esi::ExchangeContractsReq {
                    characters: Vec::new(),
                    corporations: vec![self.buyback_corp.clone()],
                    active_only: true,
                    include_items: req.include_items,
                    structure_token: self.buyback_contract_structure_token.clone(),
                }
                .into(),
            )
            .await
            .map_err(|e| Error::WeveEsi(e))?
            .output
            .inner;

        let mut check_futs = Vec::new();
        let mut buy_check_futs = Vec::new();

        let mut rep = pb::item_configurator::BuybackContractsRep {
            contracts: esi_contracts
                .into_iter()
                .filter_map(|c| match self.buyback_contract_regex.find(&c.description) {
                    None => None,
                    Some(hash_code) => {
                        let hash_code = hash_code.as_str().to_string();
                        match (req.include_check, req.include_buy) {
                            (true, true) => buy_check_futs
                                .push(self.bb_check_then_buy(hash_code.clone(), &req.language)),
                            (true, false) => {
                                check_futs.push(self.bb_check(hash_code.clone(), &req.language))
                            }
                            _ => (),
                            // (false, true) => (),
                            // (false, false) => (),
                        };
                        Some(pb::item_configurator::BuybackContract {
                            esi_contract: Some(c),
                            check_contract: None,
                            buy_contract: None,
                            hash_code: hash_code.clone(),
                        })
                    }
                })
                .collect(),
            refresh_token: new_token,
            authorized: true,
        };

        if req.include_check {
            if req.include_buy {
                for (i, f) in buy_check_futs.into_iter().enumerate() {
                    let (check_rep, buy_rep) = f.await?;
                    rep.contracts[i].check_contract = Some(check_rep);
                    rep.contracts[i].buy_contract = Some(buy_rep);
                }
            } else {
                for (i, f) in check_futs.into_iter().enumerate() {
                    let check_rep = f.await?;
                    rep.contracts[i].check_contract = Some(check_rep);
                }
            }
        }

        Ok(rep)
    }
}
