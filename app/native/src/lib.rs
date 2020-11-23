extern crate netlink_packet_core;
extern crate netlink_sys;
extern crate packet;

mod msg;

use neon::prelude::*;

static mut NETLINK_SOCKET: Option<msg::Socket> = None;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn connect_kernel(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        NETLINK_SOCKET = Some(msg::Socket::new(msg::NETLINK_PROTOCOL as isize));
    }
    Ok(cx.undefined())
}

fn filter_rules(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let socket: &Option<msg::Socket>;
    unsafe {
        socket = &NETLINK_SOCKET;
    }

    let obj = cx.argument::<JsObject>(0)?;
    let t = obj.get(&mut cx, "sourceIP")?
        .downcast::<JsNumber>()
        .or_throw(&mut cx)?.value();
    
    let rules = packet::packets::FilterRule {
        source_ip: obj.get(&mut cx, "sourceIP")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u32,
        source_mask: obj.get(&mut cx, "sourceMask")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u32,
        source_port: obj.get(&mut cx, "sourcePort")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u16,
        dest_ip: obj.get(&mut cx, "destIP")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u32,
        dest_mask: obj.get(&mut cx, "destMask")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u32,
        dest_port: obj.get(&mut cx, "destPort")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u16,
        protocol: obj.get(&mut cx, "protocol")?
            .downcast::<JsNumber>()
            .or_throw(&mut cx)?.value() as u8,

    };


    if let Some(socket) = socket {
        socket.send(0, 0, rules);
        Ok(cx.undefined())
    }
    else {
        cx.throw_error("Netlink not connected to kernel.")
    }
}

fn capture_packet(mut cx: FunctionContext) -> JsResult<JsObject> {
    let socket: &Option<msg::Socket>;
    unsafe {
        socket = &NETLINK_SOCKET;
    }

    if let Some(socket) = socket {
        let packet = socket.recv::<packet::packets::CapturedPacket>();
        let obj = JsObject::new(&mut cx);
        let src_ip = cx.number(packet.source_ip.to_be());
        let src_port = cx.number(packet.source_port.to_be());
        let dst_ip =  cx.number(packet.dest_ip.to_be());
        let dst_port = cx.number(packet.dest_port.to_be());
        let protocol = cx.number(packet.protocol);
        obj.set(&mut cx, "sourceIP", src_ip).unwrap();
        obj.set(&mut cx, "destIP", dst_ip).unwrap();
        obj.set(&mut cx, "sourcePort", src_port).unwrap();
        obj.set(&mut cx, "destPort", dst_port).unwrap();
        obj.set(&mut cx, "protocol", protocol).unwrap();
        let mut buffer = JsArrayBuffer::new(&mut cx, packet.payload.len() as u32).unwrap();
        {
            let lock = cx.lock();
            let data = buffer.borrow_mut(&lock);
            let slice = data.as_mut_slice::<u8>();
            slice.copy_from_slice(&packet.payload);
        }
        obj.set(&mut cx, "payload", buffer).unwrap();
        Ok(obj)
    } else {
        cx.throw_error("Netlink not connected to kernel.")
    }
}

register_module!(mut cx, { 
    cx.export_function("hello", hello);
    cx.export_function("connectKernel", connect_kernel);
    cx.export_function("filterRules", filter_rules);
    cx.export_function("capturePacket", capture_packet);
    Ok(())
});
