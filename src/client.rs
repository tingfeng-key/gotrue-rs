use crate::{
    api::{Api, EmailOrPhone},
    error::Error,
    session::Session,
    user_attributes::UserAttributes,
    user_update::UserUpdate,
};

pub struct Client {
    current_session: Option<Session>,
    auto_refresh_token: bool,
    api: Api,
}

impl Client {
    pub fn new(url: String) -> Client {
        Client {
            auto_refresh_token: true,
            current_session: None,
            api: Api::new(url),
        }
    }

    pub async fn sign_up(&mut self, email: &String, password: &String) -> Session {
        let result = self.api.sign_up(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub async fn sign_in(&mut self, email: &String, password: &String) -> Session {
        let result = self.api.sign_in(&email, &password).await;

        match result {
            Ok(session) => {
                self.current_session = Some(session.clone());
                return session;
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> bool {
        let result = self.api.send_otp(email_or_phone, should_create_user).await;

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn verify_otp<T: serde::Serialize>(&self, params: T) -> bool {
        let result = self.api.verify_otp(params).await;

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn sign_out(&self) -> bool {
        let result = match &self.current_session {
            Some(session) => self.api.sign_out(&session.access_token).await,
            None => return true,
        };

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn reset_password_for_email(&self, email: &str) -> bool {
        let result = self.api.reset_password_for_email(&email).await;

        match result {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub async fn update_user(&self, user: UserAttributes) -> Result<UserUpdate, reqwest::Error> {
        let session = match &self.current_session {
            Some(s) => s,
            None => panic!("Not logged in"),
        };

        let result = self.api.update_user(user, &session.access_token).await?;

        return Ok(result);
    }

    pub async fn refresh_session(&mut self) -> Result<Session, Error> {
        if self.current_session.is_none() {
            return Err(Error::NotAuthenticated);
        }

        let result = match &self.current_session {
            Some(session) => self.api.refresh_access_token(&session.refresh_token).await,
            None => return Err(Error::MissingRefreshToken),
        };

        let session = match result {
            Ok(session) => session,
            Err(_) => return Err(Error::InternalError),
        };

        self.current_session = Some(session.clone());

        return Ok(session);
    }
}