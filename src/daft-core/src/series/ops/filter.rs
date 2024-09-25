use common_error::{DaftError, DaftResult};

use crate::{datatypes::BooleanArray, series::Series};

impl Series {
    pub fn filter(&self, mask: &BooleanArray) -> DaftResult<Self> {
        match (self.len(), mask.len()) {
            (_, 1) => {
                if Some(true) == mask.get(0) {
                    Ok(self.clone())
                } else {
                    self.head(0)
                }
            }
            (n, m) if n == m => self.inner.filter(mask),
            _ => Err(DaftError::ValueError(format!(
                "Lengths for filter do not match, Series {} vs mask {}",
                self.len(),
                mask.len()
            ))),
        }
    }
}
