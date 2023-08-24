use crate::TrinaryLogic;

pub struct AcceptsResult {
    pub result: TrinaryLogic,
    pub reasons: Vec<String>,
}

impl AcceptsResult {
    pub fn yes(&self) -> bool {
        self.result.yes()
    }

    pub fn maybe(&self) -> bool {
        self.result.maybe()
    }

    pub fn no(&self) -> bool {
        self.result.no()
    }

    pub fn create_yes() -> Self {
        Self {
            result: TrinaryLogic::Yes,
            reasons: vec![],
        }
    }

    pub fn create_maybe() -> Self {
        Self {
            result: TrinaryLogic::Maybe,
            reasons: vec![],
        }
    }

    pub fn create_no() -> Self {
        Self {
            result: TrinaryLogic::No,
            reasons: vec![],
        }
    }

    pub fn decorate(self, callback: fn(String) -> String) -> Self {
        let mut new_reasons: Vec<String> = vec![];
        for reason in self.reasons {
            new_reasons.push(callback(reason))
        }

        Self {
            result: self.result,
            reasons: new_reasons,
        }
    }
}
