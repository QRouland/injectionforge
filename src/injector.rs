
#[cfg(all(not(feature = "frida")))]
compile_error!("No injection method is selected - please enable either dotnet (windows-only) and/or frida feature");

#[cfg(feature = "frida")]
use crate::frida_handler::attach_pid as frida_attach_pid;

#[no_mangle]
pub extern "C" fn attach(pid: u32) {
	#[cfg(feature = "frida")]
	{
		let frida_code = env!("FRIDA_CODE");
		#[cfg(windows)]
		std::thread::spawn(move || frida_attach_pid(frida_code, pid));
		#[cfg(not(windows))]
		frida_attach_pid(frida_code, pid);
	}
}

#[no_mangle]
pub extern "C" fn attach_self() {
	println!("[*] Attaching to self");
	attach(0);
}
