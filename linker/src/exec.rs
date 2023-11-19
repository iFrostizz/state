use revm::primitives::{CreateScheme, TransactTo};
use revm::{InMemoryDB, EVM};

pub struct Provider {
    evm: EVM<InMemoryDB>,
}

impl Provider {
    fn new() -> Self {
        let evm = EVM::new();
        Self { evm }
    }

    fn deploy(
        &mut self,
        data: revm::primitives::Bytes,
    ) -> Result<
        revm::primitives::ExecutionResult,
        revm::primitives::EVMError<std::convert::Infallible>,
    > {
        let evm = &mut self.evm;
        let tx = &mut evm.env.tx;
        tx.data = data;
        tx.transact_to = TransactTo::Create(CreateScheme::Create);
        evm.transact_commit()
    }
}
