mod args;

use std::ffi::{c_char, CStr};
use clap::Parser;
use args::ClapInfoHostArgs;

use clap_sys::entry::clap_plugin_entry;
use clap_sys::plugin::clap_plugin_descriptor;
use clap_sys::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use libloading::{Library, Symbol};

const UNKNOWN_FIELD: &str = "Unknown";

unsafe fn ctos(s: *const c_char) -> Result<String, ()> {
    if !s.is_null() {
        Ok(CStr::from_ptr(s).to_str().expect("Unable to convert CStr to &str!").to_string())
    } else {
        Err(())
    }
}

unsafe fn ctos_or_unknown(s: *const c_char) -> String {
    ctos(s).unwrap_or(String::from(UNKNOWN_FIELD))
}
unsafe fn print_plugin_info(descriptor: *const clap_plugin_descriptor) {
    let descriptor = *descriptor;

    // Basic plugin information
    println!("Name: {}", ctos_or_unknown(descriptor.name));
    println!("Vendor: {}", ctos_or_unknown(descriptor.vendor));
    println!("ID: {}", ctos_or_unknown(descriptor.id));

    // Versions
    println!("Version: {}", ctos_or_unknown(descriptor.version));
    let clap_version = descriptor.clap_version;
    println!("CLAP Version: {}.{}.{}", clap_version.major, clap_version.minor, clap_version.revision);

    // Other information
    if let Ok(description) = ctos(descriptor.description) {
        println!("Description: {description}");
    }
    // println!("Features: {}", ctos(descriptor.features));

    // URLs
    if let Ok(url) = ctos(descriptor.url) {
        println!("URL: {url}");
    }
    if let Ok(url) = ctos(descriptor.manual_url) {
        println!("Manual: {url}");
    }
    if let Ok(url) = ctos(descriptor.support_url) {
        println!("Support: {url}");
    }
}

fn main() {
    let args = ClapInfoHostArgs::parse();

    // Attempt to load the library
    let lib = unsafe { Library::new(&args.path) };
    let lib = match lib {
        Ok(lib) => lib,
        Err(_) => {
            eprintln!("Unable to load CLAP plugin \"{}\"!", args.path);
            return;
        }
    };

    // Grab the plugin entry point
    let entry: Symbol<*const clap_plugin_entry> = unsafe { lib.get(b"clap_entry") }.expect("Failed to load clap_entry symbol");
    let entry: *const clap_plugin_entry = *entry;

    let factory = unsafe { ((*entry).get_factory)(CLAP_PLUGIN_FACTORY_ID) } as *const clap_plugin_factory;
    let n_plugins = unsafe { ((*factory).get_plugin_count)(factory) };
    for i in 0..n_plugins {
        let descriptor = unsafe { ((*factory).get_plugin_descriptor)(factory, i) };
        unsafe { print_plugin_info(descriptor); }
        if i < n_plugins - 1 {
            println!()
        }
    }
}
