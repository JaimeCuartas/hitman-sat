use hitman::Hitman as CoreHitman;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Hitman {
    inner: CoreHitman,
}

impl Default for Hitman {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Hitman {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> Hitman {
        Hitman {
            inner: CoreHitman::default(),
        }
    }

    /// Add a new set to hit
    /// # Errors
    /// Throws an Error
    /// if more than 2^31-1 distinct elements are added
    pub fn hit(&mut self, set: &[u32]) -> Result<(), JsError> {
        self.inner.hit(set)?;
        Ok(())
    }

    /// Prevents this hitting set, and any superset of it
    /// # Errors
    /// Throws an Error
    /// if more than 2^31-1 distinct elements are added
    pub fn block(&mut self, set: &[u32]) -> Result<(), JsError> {
        self.inner.block(set)?;
        Ok(())
    }

    /// Retrieve a hitting set and undefined when
    /// no more hitting sets exist
    /// # Errors
    /// Throws an Error
    /// if SAT solver is interrupted
    pub fn get(&mut self) -> Result<Option<Vec<u32>>, JsError> {
        Ok(self.inner.get()?)
    }
}
