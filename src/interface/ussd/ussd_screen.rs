use super::ussd_request::USSDRequest;

pub struct USSDScreen {
    pub ussd_request: USSDRequest,

}

pub trait USSDScreenTrait {
    fn new(ussd_request: USSDRequest) -> Self;
}
