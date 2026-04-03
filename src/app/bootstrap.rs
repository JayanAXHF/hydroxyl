use crate::{
    app::{context::AppContext, state::AppState},
    util::result::Result,
};

pub fn build_state(context: AppContext) -> Result<AppState> {
    Ok(AppState::new(context))
}
