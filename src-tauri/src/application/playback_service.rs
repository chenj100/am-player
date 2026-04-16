use crate::application::errors::AppError;

#[derive(Default)]
pub struct PlaybackService;

impl PlaybackService {
    pub fn ensure_local_playback(&self, local_file_path: Option<&str>) -> Result<(), AppError> {
        if local_file_path.is_none() {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
