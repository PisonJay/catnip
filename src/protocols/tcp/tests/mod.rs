// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

pub mod setup;

// pub fn one_send_recv_round(
//     ctx: &mut Context,
//     buf: Bytes,
//     alice: &mut test_helpers::TestEngine,
//     alice_fd: FileDescriptor,
//     bob: &mut test_helpers::TestEngine,
//     bob_fd: FileDescriptor,
// ) {

//     // Send data from Alice to Bob
//     debug!("Sending from Alice to Bob");
//     let mut push_future = alice.tcp_push(alice_fd, buf.clone());
//     must_let!(let Poll::Ready(Ok(())) = Future::poll(Pin::new(&mut push_future), ctx));
//     alice.rt().poll_scheduler();
//     bob.receive(alice.rt().pop_frame()).unwrap();

//     // Receive it on Bob's side.
//     let mut pop_future = bob.tcp_pop(bob_fd);
//     must_let!(let Poll::Ready(Ok(received_buf)) = Future::poll(Pin::new(&mut pop_future), ctx));
//     debug_assert_eq!(received_buf, buf);

//     // Send data from Bob to Alice
//     debug!("Sending from Bob to Alice");
//     let mut push_future = bob.tcp_push(bob_fd, buf.clone());
//     must_let!(let Poll::Ready(Ok(())) = Future::poll(Pin::new(&mut push_future), ctx));
//     bob.rt().poll_scheduler();
//     alice.receive(bob.rt().pop_frame()).unwrap();

//     // Receive it on Alice's side.
//     let mut pop_future = alice.tcp_pop(alice_fd);
//     must_let!(let Poll::Ready(Ok(received_buf)) = Future::poll(Pin::new(&mut pop_future), ctx));
//     debug_assert_eq!(received_buf, buf);
// }

// #[test]
// fn tcp_loop() {
//     let mut ctx = Context::from_waker(noop_waker_ref());
//     let now = Instant::now();
//     let mut alice = test_helpers::new_alice(now);
//     let mut bob = test_helpers::new_bob(now);

//     // Establish the connection between the two peers.
//     let listen_port = ip::Port::try_from(80).unwrap();
//     let listen_addr = ipv4::Endpoint::new(test_helpers::BOB_IPV4, listen_port);

//     let listen_fd = bob.tcp_socket();
//     bob.tcp_bind(listen_fd, listen_addr).unwrap();
//     bob.tcp_listen(listen_fd, 1).unwrap();
//     let mut accept_future = bob.tcp_accept(listen_fd);

//     let alice_fd = alice.tcp_socket();
//     let mut connect_future = alice.tcp_connect(alice_fd, listen_addr);

//     // Send the SYN from Alice to Bob
//     alice.rt().poll_scheduler();
//     bob.receive(alice.rt().pop_frame()).unwrap();

//     // Send the SYN+ACK from Bob to Alice
//     bob.rt().poll_scheduler();
//     alice.receive(bob.rt().pop_frame()).unwrap();

//     // Send the ACK from Alice to Bob
//     alice.rt().poll_scheduler();
//     bob.receive(alice.rt().pop_frame()).unwrap();

//     must_let!(let Poll::Ready(Ok(bob_fd)) = Future::poll(Pin::new(&mut accept_future), &mut ctx));
//     must_let!(let Poll::Ready(Ok(())) = Future::poll(Pin::new(&mut connect_future), &mut ctx));

//     let size = 2048;
//     let mut buf = BytesMut::zeroed(size);
//     for i in 0..size {
//         buf[i] = i as u8;
//     }
//     let buf = buf.freeze();

//     one_send_recv_round(
//         &mut ctx,
//         buf.clone(),
//         &mut alice,
//         alice_fd,
//         &mut bob,
//         bob_fd,
//     );
// }
