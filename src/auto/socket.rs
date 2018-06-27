// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Credentials;
use Error;
use InetAddress;
use SocketAddress;
use SocketConnection;
use SocketFamily;
use SocketProtocol;
use SocketType;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Socket(Object<ffi::GSocket, ffi::GSocketClass>);

    match fn {
        get_type => || ffi::g_socket_get_type(),
    }
}

impl Socket {
    pub fn new(family: SocketFamily, type_: SocketType, protocol: SocketProtocol) -> Result<Socket, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_new(family.to_glib(), type_.to_glib(), protocol.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait SocketExt {
    fn accept<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<Socket, Error>;

    fn bind<P: IsA<SocketAddress>>(&self, address: &P, allow_reuse: bool) -> Result<(), Error>;

    fn check_connect_result(&self) -> Result<(), Error>;

    fn close(&self) -> Result<(), Error>;

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition;

    fn condition_timed_wait<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, timeout: i64, cancellable: P) -> Result<(), Error>;

    fn condition_wait<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P) -> Result<(), Error>;

    fn connect<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>>(&self, address: &P, cancellable: Q) -> Result<(), Error>;

    fn connection_factory_create_connection(&self) -> Option<SocketConnection>;

    fn get_available_bytes(&self) -> isize;

    fn get_blocking(&self) -> bool;

    fn get_broadcast(&self) -> bool;

    fn get_credentials(&self) -> Result<Credentials, Error>;

    fn get_family(&self) -> SocketFamily;

    fn get_keepalive(&self) -> bool;

    fn get_listen_backlog(&self) -> i32;

    fn get_local_address(&self) -> Result<SocketAddress, Error>;

    fn get_multicast_loopback(&self) -> bool;

    fn get_multicast_ttl(&self) -> u32;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_option(&self, level: i32, optname: i32) -> Result<i32, Error>;

    fn get_protocol(&self) -> SocketProtocol;

    fn get_remote_address(&self) -> Result<SocketAddress, Error>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_ttl(&self) -> u32;

    fn is_closed(&self) -> bool;

    fn is_connected(&self) -> bool;

    fn join_multicast_group<'a, P: Into<Option<&'a str>>>(&self, group: &InetAddress, source_specific: bool, iface: P) -> Result<(), Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn join_multicast_group_ssm<'a, 'b, P: Into<Option<&'a InetAddress>>, Q: Into<Option<&'b str>>>(&self, group: &InetAddress, source_specific: P, iface: Q) -> Result<(), Error>;

    fn leave_multicast_group<'a, P: Into<Option<&'a str>>>(&self, group: &InetAddress, source_specific: bool, iface: P) -> Result<(), Error>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn leave_multicast_group_ssm<'a, 'b, P: Into<Option<&'a InetAddress>>, Q: Into<Option<&'b str>>>(&self, group: &InetAddress, source_specific: P, iface: Q) -> Result<(), Error>;

    fn listen(&self) -> Result<(), Error>;

    fn set_blocking(&self, blocking: bool);

    fn set_broadcast(&self, broadcast: bool);

    fn set_keepalive(&self, keepalive: bool);

    fn set_listen_backlog(&self, backlog: i32);

    fn set_multicast_loopback(&self, loopback: bool);

    fn set_multicast_ttl(&self, ttl: u32);

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), Error>;

    fn set_timeout(&self, timeout: u32);

    fn set_ttl(&self, ttl: u32);

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), Error>;

    fn speaks_ipv4(&self) -> bool;

    fn get_property_type(&self) -> SocketType;

    fn connect_property_blocking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_broadcast_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_keepalive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_remote_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Socket> + IsA<glib::object::Object>> SocketExt for O {
    fn accept<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<Socket, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_accept(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn bind<P: IsA<SocketAddress>>(&self, address: &P, allow_reuse: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_bind(self.to_glib_none().0, address.to_glib_none().0, allow_reuse.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn check_connect_result(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_check_connect_result(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_close(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition {
        unsafe {
            from_glib(ffi::g_socket_condition_check(self.to_glib_none().0, condition.to_glib()))
        }
    }

    fn condition_timed_wait<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, timeout: i64, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_timed_wait(self.to_glib_none().0, condition.to_glib(), timeout, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn condition_wait<'a, P: Into<Option<&'a Cancellable>>>(&self, condition: glib::IOCondition, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_condition_wait(self.to_glib_none().0, condition.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect<'a, P: IsA<SocketAddress>, Q: Into<Option<&'a Cancellable>>>(&self, address: &P, cancellable: Q) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_connect(self.to_glib_none().0, address.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connection_factory_create_connection(&self) -> Option<SocketConnection> {
        unsafe {
            from_glib_full(ffi::g_socket_connection_factory_create_connection(self.to_glib_none().0))
        }
    }

    fn get_available_bytes(&self) -> isize {
        unsafe {
            ffi::g_socket_get_available_bytes(self.to_glib_none().0)
        }
    }

    fn get_blocking(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_blocking(self.to_glib_none().0))
        }
    }

    fn get_broadcast(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_broadcast(self.to_glib_none().0))
        }
    }

    fn get_credentials(&self) -> Result<Credentials, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_credentials(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_get_family(self.to_glib_none().0))
        }
    }

    fn get_keepalive(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_keepalive(self.to_glib_none().0))
        }
    }

    fn get_listen_backlog(&self) -> i32 {
        unsafe {
            ffi::g_socket_get_listen_backlog(self.to_glib_none().0)
        }
    }

    fn get_local_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_local_address(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_multicast_loopback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_get_multicast_loopback(self.to_glib_none().0))
        }
    }

    fn get_multicast_ttl(&self) -> u32 {
        unsafe {
            ffi::g_socket_get_multicast_ttl(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_option(&self, level: i32, optname: i32) -> Result<i32, Error> {
        unsafe {
            let mut value = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_get_option(self.to_glib_none().0, level, optname, &mut value, &mut error);
            if error.is_null() { Ok(value) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(ffi::g_socket_get_protocol(self.to_glib_none().0))
        }
    }

    fn get_remote_address(&self) -> Result<SocketAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_get_remote_address(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_get_socket_type(self.to_glib_none().0))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe {
            ffi::g_socket_get_timeout(self.to_glib_none().0)
        }
    }

    fn get_ttl(&self) -> u32 {
        unsafe {
            ffi::g_socket_get_ttl(self.to_glib_none().0)
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_is_closed(self.to_glib_none().0))
        }
    }

    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_is_connected(self.to_glib_none().0))
        }
    }

    fn join_multicast_group<'a, P: Into<Option<&'a str>>>(&self, group: &InetAddress, source_specific: bool, iface: P) -> Result<(), Error> {
        let iface = iface.into();
        let iface = iface.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group(self.to_glib_none().0, group.to_glib_none().0, source_specific.to_glib(), iface.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn join_multicast_group_ssm<'a, 'b, P: Into<Option<&'a InetAddress>>, Q: Into<Option<&'b str>>>(&self, group: &InetAddress, source_specific: P, iface: Q) -> Result<(), Error> {
        let source_specific = source_specific.into();
        let source_specific = source_specific.to_glib_none();
        let iface = iface.into();
        let iface = iface.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_join_multicast_group_ssm(self.to_glib_none().0, group.to_glib_none().0, source_specific.0, iface.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn leave_multicast_group<'a, P: Into<Option<&'a str>>>(&self, group: &InetAddress, source_specific: bool, iface: P) -> Result<(), Error> {
        let iface = iface.into();
        let iface = iface.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group(self.to_glib_none().0, group.to_glib_none().0, source_specific.to_glib(), iface.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn leave_multicast_group_ssm<'a, 'b, P: Into<Option<&'a InetAddress>>, Q: Into<Option<&'b str>>>(&self, group: &InetAddress, source_specific: P, iface: Q) -> Result<(), Error> {
        let source_specific = source_specific.into();
        let source_specific = source_specific.to_glib_none();
        let iface = iface.into();
        let iface = iface.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_leave_multicast_group_ssm(self.to_glib_none().0, group.to_glib_none().0, source_specific.0, iface.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn listen(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listen(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_blocking(&self, blocking: bool) {
        unsafe {
            ffi::g_socket_set_blocking(self.to_glib_none().0, blocking.to_glib());
        }
    }

    fn set_broadcast(&self, broadcast: bool) {
        unsafe {
            ffi::g_socket_set_broadcast(self.to_glib_none().0, broadcast.to_glib());
        }
    }

    fn set_keepalive(&self, keepalive: bool) {
        unsafe {
            ffi::g_socket_set_keepalive(self.to_glib_none().0, keepalive.to_glib());
        }
    }

    fn set_listen_backlog(&self, backlog: i32) {
        unsafe {
            ffi::g_socket_set_listen_backlog(self.to_glib_none().0, backlog);
        }
    }

    fn set_multicast_loopback(&self, loopback: bool) {
        unsafe {
            ffi::g_socket_set_multicast_loopback(self.to_glib_none().0, loopback.to_glib());
        }
    }

    fn set_multicast_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_multicast_ttl(self.to_glib_none().0, ttl);
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn set_option(&self, level: i32, optname: i32, value: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_set_option(self.to_glib_none().0, level, optname, value, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_set_timeout(self.to_glib_none().0, timeout);
        }
    }

    fn set_ttl(&self, ttl: u32) {
        unsafe {
            ffi::g_socket_set_ttl(self.to_glib_none().0, ttl);
        }
    }

    fn shutdown(&self, shutdown_read: bool, shutdown_write: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_shutdown(self.to_glib_none().0, shutdown_read.to_glib(), shutdown_write.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn speaks_ipv4(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_speaks_ipv4(self.to_glib_none().0))
        }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_blocking_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::blocking",
                transmute(notify_blocking_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_broadcast_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::broadcast",
                transmute(notify_broadcast_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::family",
                transmute(notify_family_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_keepalive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::keepalive",
                transmute(notify_keepalive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::listen-backlog",
                transmute(notify_listen_backlog_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-address",
                transmute(notify_local_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_multicast_loopback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::multicast-loopback",
                transmute(notify_multicast_loopback_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_multicast_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::multicast-ttl",
                transmute(notify_multicast_ttl_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_remote_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::remote-address",
                transmute(notify_remote_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::timeout",
                transmute(notify_timeout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ttl_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ttl",
                transmute(notify_ttl_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_blocking_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_broadcast_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_family_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_keepalive_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_listen_backlog_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_local_address_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_multicast_loopback_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_multicast_ttl_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_remote_address_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_timeout_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ttl_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::GSocket, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Socket> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Socket::from_glib_borrow(this).downcast_unchecked())
}
