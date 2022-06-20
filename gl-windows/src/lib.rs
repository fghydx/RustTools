#![cfg(windows)]
extern crate windows_service;
/// # Example
///[cfg(windows)]
///fn main(){
///    let start = ||{
///        println!("run")
///    };
///    let stop = ||{
///        println!("stop")
///    };
///    let a = gl_windows::windowservice::create_service("abc",
///                                                     &String::from("bcd"),&String::from("cde"),Some(start),Some(stop));
///    match a {
///        Ok(T)=>println!("ok"),
///        Err(e)=>eprintln!("{}",e)
///    }
///}
pub mod windowservice {
    use std::ffi::{OsString};
    use std::{env, thread};
    use windows_service::{service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType}, service_manager::{ServiceManager, ServiceManagerAccess}};
    use std::time::Duration;
    use std::option::Option::Some;
    use std::sync::mpsc;

    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher,
    };

    define_windows_service!(ffi_service_main, my_service_main);
    static mut PSTART: Option<fn()> = None;
    static mut PSTOP: Option<fn()> = None;
    static mut SERVICE_NAME: &str = "12";

    fn my_service_main(_arguments: Vec<OsString>) {
        let _res = run_service();
            // Handle the error, by logging or something.
    }

    fn start()-> windows_service::Result<()>{
        let tt = unsafe {SERVICE_NAME};
        if let Err(e) = service_dispatcher::start(tt, ffi_service_main){
            println!("pub fn start(){:#?}",e);
        }
        Ok(())
    }

    fn run_service()-> windows_service::Result<()>{
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };
        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let tmp = service_control_handler::register(unsafe {SERVICE_NAME}, event_handler);
        match tmp {
            Err(e)=>{println!("{:#?}",e)},
            Ok(status_handle) => {
                if let Err(e) = status_handle.set_service_status(ServiceStatus {
                service_type: ServiceType::OWN_PROCESS,
                current_state: ServiceState::Running,
                controls_accepted: ServiceControlAccept::STOP,
                exit_code: ServiceExitCode::Win32(0),
                checkpoint: 0,
                wait_hint: Duration::default(),
                process_id: None,
            }){
                    println!("status_handle.set_service_status0 {:#?}",e);
                };
                println!("after set_service_status register");
                unsafe {
                    if let Some(s) = PSTART {
                        thread::spawn(move ||{
                            s();
                        });
                        // s();
                    }
                }
                println!("after PSTART");
                loop {
                    // Poll shutdown event.
                    match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                        // Break the loop either upon stop or channel disconnect
                        Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                        // Continue work if no events were received within the timeout
                        Err(mpsc::RecvTimeoutError::Timeout) => (),
                    };
                }
                println!("after loop");
                unsafe {
                    if let Some(s) = PSTOP {
                        s();
                    }
                }
                println!("after PSTOP");
                // Tell the system that service has stopped.
                status_handle.set_service_status(ServiceStatus {
                    service_type: ServiceType::OWN_PROCESS,
                    current_state: ServiceState::Stopped,
                    controls_accepted: ServiceControlAccept::empty(),
                    exit_code: ServiceExitCode::Win32(0),
                    checkpoint: 0,
                    wait_hint: Duration::default(),
                    process_id: None,
                })?;
                println!("after set_service_status2");}
        }
        println!("after status_handle register");
        // Tell the system that service is running
        Ok(())
    }

    pub fn create_service(name:&'static str, display_name:&String, start:Option<fn()>, stop:Option<fn()>) -> windows_service::Result<()>{
        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

        unsafe {
            PSTART = start;
            PSTOP = stop;
            SERVICE_NAME = name;
        }
        if env::args().len()==1 {
            println!("执行");
            return self::start()
        }
        for argument in env::args() {
            if argument.to_lowercase() == "install" {
                println!("{},{}", "installing", name);
                if let Err(e) = createsvr(&service_manager, name, display_name){
                    return Err(e)
                }
                println!("install success");
                break
            } else if argument.to_lowercase() == "uninstall" {
                println!("uninstalling");
                let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
                let service = service_manager.open_service(name, service_access)?;
                let service_status = service.query_status()?;
                if service_status.current_state != ServiceState::Stopped {
                    service.stop()?;
                    // Wait for service to stop
                    thread::sleep(Duration::from_secs(1));
                }
                service.delete()?;
                println!("uninstall success");
                return Ok(())
            }
        }
        println!("createservice!{}",name);
        Ok(())
    }

    fn createsvr(service_manager:&ServiceManager, name: &str, display_name: &String) -> windows_service::Result<()> {
        let exename = std::env::current_exe().unwrap();
        let service_info = ServiceInfo {
            name: OsString::from(name),
            display_name: OsString::from(display_name),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: exename,
            launch_arguments: vec![],
            dependencies: vec![],
            account_name: None, // run as System
            account_password: None,
        };
        let _service = service_manager.create_service(&service_info, ServiceAccess::empty())?;
        Ok(())
    }

}
