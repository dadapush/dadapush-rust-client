use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    da_da_push_message_api: Box<::apis::DaDaPushMessageApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            da_da_push_message_api: Box::new(::apis::DaDaPushMessageApiClient::new(rc.clone())),
        }
    }

    pub fn da_da_push_message_api(&self) -> &::apis::DaDaPushMessageApi{
        self.da_da_push_message_api.as_ref()
    }

}
