// Provides the functionality of __declspec(uuid) and __uuidof()
use winapi;

pub trait DeclspecUUID {
    // TODO(zeffron 2016-08-15): Replace the winapi type in the return value
    fn uuid() -> winapi::GUID;
}