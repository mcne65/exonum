// Copyright 2017 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A special service which generates transactions on `handle_commit` events.

use exonum::blockchain::{Service, ServiceContext, Transaction, ExecutionStatus};
use exonum::messages::{RawTransaction, Message};
use exonum::storage::{Fork, Snapshot};
use exonum::crypto::{Hash, Signature};
use exonum::encoding;
use exonum::helpers::Height;

const SERVICE_ID: u16 = 512;
const TX_AFTER_COMMIT_ID: u16 = 1;

message! {
    struct TxAfterCommit {
        const TYPE = SERVICE_ID;
        const ID = TX_AFTER_COMMIT_ID;

        height: Height,
    }
}

impl Transaction for TxAfterCommit {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, _fork: &mut Fork) -> ExecutionStatus {
        Ok(())
    }
}

pub struct HandleCommitService;

impl Service for HandleCommitService {
    fn service_name(&self) -> &'static str {
        "handle_commit"
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        Vec::new()
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx: Box<Transaction> = match raw.message_type() {
            TX_AFTER_COMMIT_ID => Box::new(TxAfterCommit::from_raw(raw)?),
            _ => {
                return Err(encoding::Error::IncorrectMessageType {
                    message_type: raw.message_type(),
                });
            }
        };
        Ok(tx)
    }

    fn handle_commit(&self, context: &ServiceContext) {
        let tx = TxAfterCommit::new_with_signature(context.height(), &Signature::zero());
        context.transaction_sender().send(Box::new(tx)).unwrap();
    }
}
