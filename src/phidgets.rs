pub mod common;
pub mod network;
pub mod phidget22;
pub mod temperature;
pub mod lcd;

#[derive(Copy, Clone)]
pub struct Phidget {
    handle: phidget22::PhidgetHandle,
}

pub trait PhidgetTraits {
    // fn new() -> Phidget;
    fn get_return_code_string(code: u32) -> String;
    fn set_is_remote(&self, is_remote: bool) -> phidget22::PhidgetReturnCode;
    fn set_device_serial_number(&self, serial: i32) -> phidget22::PhidgetReturnCode;
    fn set_hub_port(&self, port: i32) -> phidget22::PhidgetReturnCode;
    fn open_wait_for_attachment(&self, timeout: u32) -> phidget22::PhidgetReturnCode;
}

impl PhidgetTraits for Phidget {
    // fn new() -> Phidget {
    //     return Phidget { handle: None };
    // }
    fn get_return_code_string(code: u32) -> String {
        let res = match code {
            phidget22::PhidgetReturnCode_EPHIDGET_OK => "EPHIDGET_OK",
            phidget22::PhidgetReturnCode_EPHIDGET_PERM => "EPHIDGET_PERM",
            phidget22::PhidgetReturnCode_EPHIDGET_NOENT => "EPHIDGET_NOENT",
            phidget22::PhidgetReturnCode_EPHIDGET_TIMEOUT => "EPHIDGET_TIMEOUT",
            phidget22::PhidgetReturnCode_EPHIDGET_KEEPALIVE => "EPHIDGET_KEEPALIVE",
            phidget22::PhidgetReturnCode_EPHIDGET_INTERRUPTED => "EPHIDGET_INTERRUPTED",
            phidget22::PhidgetReturnCode_EPHIDGET_IO => "EPHIDGET_IO",
            phidget22::PhidgetReturnCode_EPHIDGET_NOMEMORY => "EPHIDGET_NOMEMORY",
            phidget22::PhidgetReturnCode_EPHIDGET_ACCESS => "EPHIDGET_ACCESS",
            phidget22::PhidgetReturnCode_EPHIDGET_FAULT => "EPHIDGET_FAULT",
            phidget22::PhidgetReturnCode_EPHIDGET_BUSY => "EPHIDGET_BUSY",
            phidget22::PhidgetReturnCode_EPHIDGET_EXIST => "EPHIDGET_EXIST",
            phidget22::PhidgetReturnCode_EPHIDGET_NOTDIR => "EPHIDGET_NOTDIR",
            phidget22::PhidgetReturnCode_EPHIDGET_ISDIR => "EPHIDGET_ISDIR",
            phidget22::PhidgetReturnCode_EPHIDGET_INVALID => "EPHIDGET_INVALID",
            phidget22::PhidgetReturnCode_EPHIDGET_NFILE => "EPHIDGET_NFILE",
            phidget22::PhidgetReturnCode_EPHIDGET_MFILE => "EPHIDGET_MFILE",
            phidget22::PhidgetReturnCode_EPHIDGET_NOSPC => "EPHIDGET_NOSPC",
            phidget22::PhidgetReturnCode_EPHIDGET_FBIG => "EPHIDGET_FBIG",
            phidget22::PhidgetReturnCode_EPHIDGET_ROFS => "EPHIDGET_ROFS",
            phidget22::PhidgetReturnCode_EPHIDGET_RO => "EPHIDGET_RO",
            phidget22::PhidgetReturnCode_EPHIDGET_UNSUPPORTED => "EPHIDGET_UNSUPPORTED",
            phidget22::PhidgetReturnCode_EPHIDGET_INVALIDARG => "EPHIDGET_INVALIDARG",
            phidget22::PhidgetReturnCode_EPHIDGET_AGAIN => "EPHIDGET_AGAIN",
            phidget22::PhidgetReturnCode_EPHIDGET_NOTEMPTY => "EPHIDGET_NOTEMPTY",
            phidget22::PhidgetReturnCode_EPHIDGET_UNEXPECTED => "EPHIDGET_UNEXPECTED",
            phidget22::PhidgetReturnCode_EPHIDGET_DUPLICATE => "EPHIDGET_DUPLICATE",
            phidget22::PhidgetReturnCode_EPHIDGET_BADPASSWORD => "EPHIDGET_BADPASSWORD",
            phidget22::PhidgetReturnCode_EPHIDGET_NETUNAVAIL => "EPHIDGET_NETUNAVAIL",
            phidget22::PhidgetReturnCode_EPHIDGET_CONNREF => "EPHIDGET_CONNREF",
            phidget22::PhidgetReturnCode_EPHIDGET_CONNRESET => "EPHIDGET_CONNRESET",
            phidget22::PhidgetReturnCode_EPHIDGET_HOSTUNREACH => "EPHIDGET_HOSTUNREACH",
            phidget22::PhidgetReturnCode_EPHIDGET_NODEV => "EPHIDGET_NODEV",
            phidget22::PhidgetReturnCode_EPHIDGET_WRONGDEVICE => "EPHIDGET_WRONGDEVICE",
            phidget22::PhidgetReturnCode_EPHIDGET_PIPE => "EPHIDGET_PIPE",
            phidget22::PhidgetReturnCode_EPHIDGET_RESOLV => "EPHIDGET_RESOLV",
            phidget22::PhidgetReturnCode_EPHIDGET_UNKNOWNVAL => "EPHIDGET_UNKNOWNVAL",
            phidget22::PhidgetReturnCode_EPHIDGET_NOTATTACHED => "EPHIDGET_NOTATTACHED",
            phidget22::PhidgetReturnCode_EPHIDGET_INVALIDPACKET => "EPHIDGET_INVALIDPACKET",
            phidget22::PhidgetReturnCode_EPHIDGET_2BIG => "EPHIDGET_2BIG",
            phidget22::PhidgetReturnCode_EPHIDGET_BADVERSION => "EPHIDGET_BADVERSION",
            phidget22::PhidgetReturnCode_EPHIDGET_CLOSED => "EPHIDGET_CLOSED",
            phidget22::PhidgetReturnCode_EPHIDGET_NOTCONFIGURED => "EPHIDGET_NOTCONFIGURED",
            phidget22::PhidgetReturnCode_EPHIDGET_EOF => "EPHIDGET_EOF",
            phidget22::PhidgetReturnCode_EPHIDGET_FAILSAFE => "EPHIDGET_FAILSAFE",
            _ => "EPHIDGET_UNKNOWN",
        };
        //put string on the heap
        return String::from(res);
    }
    fn set_is_remote(&self, is_remote: bool) -> phidget22::PhidgetReturnCode {
        //convert bool to i32
        let is_remote_i32 = if is_remote { 1 } else { 0 };
        let rc: u32;
        unsafe {
            rc = phidget22::Phidget_setIsRemote(self.handle, is_remote_i32);
        }
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong when calling setIsRemote: {}",
                Self::get_return_code_string(rc)
            );
        }
        return rc;
    }
    fn set_device_serial_number(&self, serial: i32) -> phidget22::PhidgetReturnCode {
        let rc: u32;
        unsafe {
            rc = phidget22::Phidget_setDeviceSerialNumber(self.handle, serial);
        }
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong when calling setDeviceSerialNumber: {}",
                Self::get_return_code_string(rc)
            );
        }
        return rc;
    }
    fn set_hub_port(&self, port: i32) -> phidget22::PhidgetReturnCode {
        let rc: u32;
        unsafe {
            rc = phidget22::Phidget_setHubPort(self.handle, port);
        }
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong when calling setHubPort: {}",
                Self::get_return_code_string(rc)
            );
        }
        return rc;
    }
    fn open_wait_for_attachment(&self, timeout: u32) -> phidget22::PhidgetReturnCode {
        let rc: u32;
        unsafe {
            rc = phidget22::Phidget_openWaitForAttachment(self.handle, timeout);
        }
        if rc != phidget22::PhidgetReturnCode_EPHIDGET_OK {
            println!(
                "Something went wrong when calling setHubPort: {}",
                Self::get_return_code_string(rc)
            );
        }
        return rc;
    }
}
