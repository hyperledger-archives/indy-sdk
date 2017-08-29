//
//  ZeroMQTests.m
//  ZeroMQ_Test
//
//  Created by Kirill Neznamov on 17/04/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//
#define ZMQ_BUILD_DRAFT_API
#import <Foundation/Foundation.h>
#import "ZeroMQTests.h"
#include "../Pods/libzmq/src/platform.hpp"
#include <zmq.h>
#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <vector>
#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include "zhelpers.h"

typedef int raw_socket;

//  This defines the settle time used in tests; raise this if we
//  get test failures on slower systems due to binds/connects not
//  settled. Tested to work reliably at 1 msec on a fast PC.

#define SETTLE_TIME 1000         //  In msec

#undef NDEBUG
#include <time.h>
#include <assert.h>
#include <stdarg.h>
#include <string>
#include <string.h>

#   include <pthread.h>
#   include <unistd.h>
#   include <signal.h>
#   include <stdlib.h>
#   include <sys/wait.h>
#   include <sys/socket.h>
#   include <netinet/in.h>
#   include <arpa/inet.h>

//  Bounce a message from client to server and back
//  For REQ/REP or DEALER/DEALER pairs only
void
bounce (void *server, void *client)
{
    const char *content = "12345678ABCDEFGH12345678abcdefgh";
    
    //  Send message from client to server
    int rc = zmq_send (client, content, 32, ZMQ_SNDMORE);
    assert (rc == 32);
    rc = zmq_send (client, content, 32, 0);
    assert (rc == 32);
    
    //  Receive message at server side
    char buffer [32];
    rc = zmq_recv (server, buffer, 32, 0);
    assert (rc == 32);
    //  Check that message is still the same
    assert (memcmp (buffer, content, 32) == 0);
    int rcvmore;
    size_t sz = sizeof (rcvmore);
    rc = zmq_getsockopt (server, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (server, buffer, 32, 0);
    assert (rc == 32);
    //  Check that message is still the same
    assert (memcmp (buffer, content, 32) == 0);
    rc = zmq_getsockopt (server, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
    
    //  Send two parts back to client
    rc = zmq_send (server, buffer, 32, ZMQ_SNDMORE);
    assert (rc == 32);
    rc = zmq_send (server, buffer, 32, 0);
    assert (rc == 32);
    
    //  Receive the two parts at the client side
    rc = zmq_recv (client, buffer, 32, 0);
    assert (rc == 32);
    //  Check that message is still the same
    assert (memcmp (buffer, content, 32) == 0);
    rc = zmq_getsockopt (client, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (client, buffer, 32, 0);
    assert (rc == 32);
    //  Check that message is still the same
    assert (memcmp (buffer, content, 32) == 0);
    rc = zmq_getsockopt (client, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
}

//  Same as bounce, but expect messages to never arrive
//  for security or subscriber reasons.
void
expect_bounce_fail (void *server, void *client)
{
    const char *content = "12345678ABCDEFGH12345678abcdefgh";
    char buffer [32];
    int timeout = 250;
    
    //  Send message from client to server
    int rc = zmq_setsockopt (client, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_send (client, content, 32, ZMQ_SNDMORE);
    assert ((rc == 32) || ((rc == -1) && (errno == EAGAIN)));
    rc = zmq_send (client, content, 32, 0);
    assert ((rc == 32) || ((rc == -1) && (errno == EAGAIN)));
    
    //  Receive message at server side (should not succeed)
    rc = zmq_setsockopt (server, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_recv (server, buffer, 32, 0);
    assert (rc == -1);
    assert (zmq_errno () == EAGAIN);
    
    //  Send message from server to client to test other direction
    //  If connection failed, send may block, without a timeout
    rc = zmq_setsockopt (server, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_send (server, content, 32, ZMQ_SNDMORE);
    assert (rc == 32 || (rc == -1 && zmq_errno () == EAGAIN));
    rc = zmq_send (server, content, 32, 0);
    assert (rc == 32 || (rc == -1 && zmq_errno () == EAGAIN));
    
    //  Receive message at client side (should not succeed)
    rc = zmq_setsockopt (client, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_recv (client, buffer, 32, 0);
    assert (rc == -1);
    assert (zmq_errno () == EAGAIN);
}

//  Convert C string to 0MQ string and send to socket
int
s_send (void *socket, const char *string) {
    int size = zmq_send (socket, string, strlen (string), 0);
    return size;
}

//  Sends string as 0MQ string, as multipart non-terminal
int
s_sendmore (void *socket, const char *string) {
    int size = zmq_send (socket, string, strlen (string), ZMQ_SNDMORE);
    return size;
}

#define streq(s1,s2)    (!strcmp ((s1), (s2)))
#define strneq(s1,s2)   (strcmp ((s1), (s2)))

const char *SEQ_END = (const char *) 1;

//  Sends a message composed of frames that are C strings or null frames.
//  The list must be terminated by SEQ_END.
//  Example: s_send_seq (req, "ABC", 0, "DEF", SEQ_END);

void
s_send_seq (void *socket, ...)
{
    va_list ap;
    va_start (ap, socket);
    const char * data = va_arg (ap, const char *);
    while (true)
    {
        const char * prev = data;
        data = va_arg (ap, const char *);
        bool end = data == SEQ_END;
        
        if (!prev) {
            int rc = zmq_send (socket, 0, 0, end ? 0 : ZMQ_SNDMORE);
            assert (rc != -1);
        }
        else {
            int rc = zmq_send (socket, prev, strlen (prev)+1, end ? 0 : ZMQ_SNDMORE);
            assert (rc != -1);
        }
        if (end)
            break;
    }
    va_end (ap);
}

//  Receives message a number of frames long and checks that the frames have
//  the given data which can be either C strings or 0 for a null frame.
//  The list must be terminated by SEQ_END.
//  Example: s_recv_seq (rep, "ABC", 0, "DEF", SEQ_END);

void
s_recv_seq (void *socket, ...)
{
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    int more;
    size_t more_size = sizeof(more);
    
    va_list ap;
    va_start (ap, socket);
    const char * data = va_arg (ap, const char *);
    
    while (true) {
        int rc = zmq_msg_recv (&msg, socket, 0);
        assert (rc != -1);
        
        if (!data)
            assert (zmq_msg_size (&msg) == 0);
        else
            assert (strcmp (data, (const char *)zmq_msg_data (&msg)) == 0);
        
        data = va_arg (ap, const char *);
        bool end = data == SEQ_END;
        
        rc = zmq_getsockopt (socket, ZMQ_RCVMORE, &more, &more_size);
        assert (rc == 0);
        
        assert (!more == end);
        if (end)
            break;
    }
    va_end (ap);
    
    zmq_msg_close (&msg);
}


//  Sets a zero linger period on a socket and closes it.
void
close_zero_linger (void *socket)
{
    int linger = 0;
    int rc = zmq_setsockopt (socket, ZMQ_LINGER, &linger, sizeof(linger));
    assert (rc == 0 || errno == ETERM);
    rc = zmq_close (socket);
    assert (rc == 0);
}

void
setup_test_environment (void)
{

}

//  Provide portable millisecond sleep
//  http://www.cplusplus.com/forum/unices/60161/
//  http://en.cppreference.com/w/cpp/thread/sleep_for

void
msleep (int milliseconds)
{
#ifdef ZMQ_HAVE_WINDOWS
    Sleep (milliseconds);
#else
    usleep (static_cast <useconds_t> (milliseconds) * 1000);
#endif
}

// check if IPv6 is available (0/false if not, 1/true if it is)
// only way to reliably check is to actually open a socket and try to bind it
int
is_ipv6_available(void)
{
#if defined (ZMQ_HAVE_WINDOWS) && (_WIN32_WINNT < 0x0600)
    return 0;
#else
    int rc, ipv6 = 1;
    struct sockaddr_in6 test_addr;
    
    memset (&test_addr, 0, sizeof (test_addr));
    test_addr.sin6_family = AF_INET6;
    inet_pton (AF_INET6, "::1", &(test_addr.sin6_addr));
    
#ifdef ZMQ_HAVE_WINDOWS
    SOCKET fd = socket (AF_INET6, SOCK_STREAM, IPPROTO_IP);
    if (fd == INVALID_SOCKET)
        ipv6 = 0;
    else {
        setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, (const char *)&ipv6, sizeof(int));
        rc = setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, (const char *)&ipv6, sizeof(int));
        if (rc == SOCKET_ERROR)
            ipv6 = 0;
        else {
            rc = bind (fd, (struct sockaddr *)&test_addr, sizeof (test_addr));
            if (rc == SOCKET_ERROR)
                ipv6 = 0;
        }
        closesocket (fd);
    }
#else
    int fd = socket (AF_INET6, SOCK_STREAM, IPPROTO_IP);
    if (fd == -1)
        ipv6 = 0;
    else {
        setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &ipv6, sizeof(int));
        rc = setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &ipv6, sizeof(int));
        if (rc != 0)
            ipv6 = 0;
        else {
            rc = bind (fd, (struct sockaddr *)&test_addr, sizeof (test_addr));
            if (rc != 0)
                ipv6 = 0;
        }
        close (fd);
    }
#endif
    
    return ipv6;
#endif // _WIN32_WINNT < 0x0600
}


int test_bind_after_connect_tcp (void)
{
    NSLog(@"- test_bind_after_connect_tcp");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_DEALER);
    assert (sb);
    
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    
    int rc = zmq_connect (sc, "tcp://127.0.0.1:7722");
    assert (rc == 0);
    
    rc = zmq_send_const (sc, "foobar", 6, 0);
    assert (rc == 6);
    
    rc = zmq_send_const (sc, "baz", 3, 0);
    assert (rc == 3);
    
    rc = zmq_send_const (sc, "buzz", 4, 0);
    assert (rc == 4);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:7722");
    assert (rc == 0);
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 6);
    void *data = zmq_msg_data (&msg);
    assert (memcmp ("foobar", data, 6) == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 3);
    data = zmq_msg_data (&msg);
    assert (memcmp ("baz", data, 3) == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 4);
    data = zmq_msg_data (&msg);
    assert (memcmp ("buzz", data, 4) == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_conflate (void)
{
    NSLog(@"- test_conflate");

    const char *bind_to = "tcp://127.0.0.1:5555";
    
    int rc;
    
    void* ctx = zmq_init (1);
    assert (ctx);
    
    void* s_in = zmq_socket (ctx, ZMQ_PULL);
    assert (s_in);
    
    int conflate = 1;
    rc = zmq_setsockopt (s_in, ZMQ_CONFLATE, &conflate, sizeof(conflate));
    assert (rc == 0);
    
    rc = zmq_bind (s_in, bind_to);
    assert (rc == 0);
    
    void* s_out = zmq_socket (ctx, ZMQ_PUSH);
    assert (s_out);
    
    rc = zmq_connect (s_out, bind_to);
    assert (rc == 0);
    
    int message_count = 20;
    for (int j = 0; j < message_count; ++j) {
        rc = zmq_send(s_out, (void*)&j, sizeof(int), 0);
        if (rc < 0) {
            printf ("error in zmq_sendmsg: %s\n", zmq_strerror (errno));
            return -1;
        }
    }
    msleep (SETTLE_TIME);
    
    int payload_recved = 0;
    rc = zmq_recv (s_in, (void*)&payload_recved, sizeof(int), 0);
    assert (rc > 0);
    assert (payload_recved == message_count - 1);
    
    rc = zmq_close (s_in);
    assert (rc == 0);
    
    rc = zmq_close (s_out);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_abstract_ipc (void)
{
    NSLog(@"- test_abstract_ipc");
    
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_DEALER);
    assert (sb);
    int rc = zmq_bind (sb, "ipc://@tmp-tester");
    assert (rc == 0);
    
    char endpoint[200];
    size_t size = sizeof(endpoint);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, endpoint, &size);
    assert (rc == 0);
    rc = strncmp(endpoint, "ipc://@tmp-tester", size);
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    rc = zmq_connect (sc, "ipc://@tmp-tester");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_ancillaries(void)
{
    NSLog(@"- test_ancillaries");

    int major, minor, patch;
    
    zmq_version (&major, &minor, &patch);
    assert (major == ZMQ_VERSION_MAJOR &&
            minor == ZMQ_VERSION_MINOR &&
            patch == ZMQ_VERSION_PATCH);
    
    assert (zmq_strerror (EINVAL));
    
    return 0;
}

int test_atomics (void)
{
    NSLog(@"- test_atomics");
    void *counter = zmq_atomic_counter_new ();
    assert (zmq_atomic_counter_value (counter) == 0);
    assert (zmq_atomic_counter_inc (counter) == 0);
    assert (zmq_atomic_counter_inc (counter) == 1);
    assert (zmq_atomic_counter_inc (counter) == 2);
    assert (zmq_atomic_counter_value (counter) == 3);
    assert (zmq_atomic_counter_dec (counter) == 1);
    assert (zmq_atomic_counter_dec (counter) == 1);
    assert (zmq_atomic_counter_dec (counter) == 0);
    zmq_atomic_counter_set (counter, 2);
    assert (zmq_atomic_counter_dec (counter) == 1);
    assert (zmq_atomic_counter_dec (counter) == 0);
    zmq_atomic_counter_destroy (&counter);
    return 0;
}


// Test vector: rfc.zeromq.org/spec:32/Z85
void test__zmq_z85_encode__valid__success ()
{
    static const size_t size = 8;
    static const size_t length = size * 5 / 4;
    static const uint8_t decoded[size] = {
        0x86, 0x4F, 0xD2, 0x6F, 0xB5, 0x59, 0xF7, 0x5B
    };
    static const char expected[length + 1] = "HelloWorld";
    char out_encoded[length + 1] = { 0 };
    
    errno = 0;
    assert (zmq_z85_encode(out_encoded, decoded, size) != NULL);
    assert (streq (out_encoded, expected));
    assert (zmq_errno () == 0);
}

// Buffer length must be evenly divisible by 4 or must fail with EINVAL.
void test__zmq_z85_encode__invalid__failure (size_t size)
{
    errno = 0;
    assert (zmq_z85_encode(NULL, NULL, size) == NULL);
    assert (zmq_errno () == EINVAL);
}

// Test vector: rfc.zeromq.org/spec:32/Z85
void test__zmq_z85_decode__valid__success ()
{
    static const size_t size = 10 * 4 / 5;
    static const uint8_t expected[size] = {
        0x86, 0x4F, 0xD2, 0x6F, 0xB5, 0x59, 0xF7, 0x5B
    };
    static const char* encoded = "HelloWorld";
    uint8_t out_decoded[size] = { 0 };
    
    errno = 0;
    assert (zmq_z85_decode(out_decoded, encoded) != NULL);
    assert (zmq_errno () == 0);
    assert (memcmp (out_decoded, expected, size) == 0);
}

// Invalid input data must fail with EINVAL.
template<size_t SIZE>
void test__zmq_z85_decode__invalid__failure (const char (&encoded)[SIZE])
{
    uint8_t decoded[SIZE * 4 / 5 + 1];
    errno = 0;
    assert (zmq_z85_decode(decoded, encoded) == NULL);
    assert (zmq_errno () == EINVAL);
}


// call zmq_z85_encode, then zmq_z85_decode, and compare the results with the original
template<size_t SIZE>
void test__zmq_z85_encode__zmq_z85_decode__roundtrip(const uint8_t (&test_data)[SIZE])
{
    char test_data_z85[SIZE * 5 / 4 + 1];
    char *res1 = zmq_z85_encode(test_data_z85, test_data, SIZE);
    assert(res1 != NULL);
    
    uint8_t test_data_decoded[SIZE];
    uint8_t *res2 = zmq_z85_decode(test_data_decoded, test_data_z85);
    assert(res2 != NULL);
    
    int res3 = memcmp(test_data, test_data_decoded, SIZE);
    assert(res3 == 0);
}

// call zmq_z85_encode, then zmq_z85_decode, and compare the results with the original
template<size_t SIZE>
void test__zmq_z85_decode__zmq_z85_encode__roundtrip(const char (&test_data)[SIZE])
{
    const size_t decoded_size = (SIZE - 1) * 4 / 5;
    uint8_t test_data_decoded[decoded_size];
    uint8_t *res1 = zmq_z85_decode(test_data_decoded, test_data);
    assert(res1 != NULL);
    
    char test_data_z85[SIZE];
    char *res2 = zmq_z85_encode(test_data_z85, test_data_decoded, decoded_size);
    assert(res2 != NULL);
    
    int res3 = memcmp(test_data, test_data_z85, SIZE);
    assert(res3 == 0);
}


int test_base85 (void)
{
    NSLog(@"- test_base85");

    test__zmq_z85_encode__valid__success ();
    test__zmq_z85_encode__invalid__failure (1);
    test__zmq_z85_encode__invalid__failure (42);
    
    test__zmq_z85_decode__valid__success ();
    // String length must be evenly divisible by 5 or must fail with EINVAL.
    test__zmq_z85_decode__invalid__failure ("01234567");
    test__zmq_z85_decode__invalid__failure ("0");
    
    // decode invalid data with the maximum representable value
    test__zmq_z85_decode__invalid__failure ("#####");
    
    // decode invalid data with the minimum value beyond the limit
    // "%nSc0" is 0xffffffff
    test__zmq_z85_decode__invalid__failure ("%nSc1");
    
    // decode invalid data with an invalid character in the range of valid
    // characters
    test__zmq_z85_decode__invalid__failure ("####\0047");
    
    // decode invalid data with an invalid character just below the range of valid
    // characters
    test__zmq_z85_decode__invalid__failure ("####\0200");
    
    // decode invalid data with an invalid character just above the range of valid
    // characters
    test__zmq_z85_decode__invalid__failure ("####\0037");
    
    // round-trip encoding and decoding with minimum value
    {
        const uint8_t test_data[] = {0x00, 0x00, 0x00, 0x00};
        test__zmq_z85_encode__zmq_z85_decode__roundtrip(test_data);
    }
    // round-trip encoding and decoding with maximum value
    {
        const uint8_t test_data[] = {0xff, 0xff, 0xff, 0xff};
        test__zmq_z85_encode__zmq_z85_decode__roundtrip(test_data);
    }
    
    test__zmq_z85_decode__zmq_z85_encode__roundtrip("r^/rM9M=rMToK)63O8dCvd9D<PY<7iGlC+{BiSnG");
    
    return 0;
}

int test_bind_src_address (void)
{
    NSLog(@"- test_bind_src_address");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sock = zmq_socket (ctx, ZMQ_PUB);
    assert (sock);
    
    int rc = zmq_connect (sock, "tcp://127.0.0.1:0;localhost:1234");
    assert (rc == 0);
    
    rc = zmq_connect (sock, "tcp://localhost:5555;localhost:1235");
    assert (rc == 0);
    
    rc = zmq_connect (sock, "tcp://lo:5555;localhost:1235");
    assert (rc == 0);
    
    rc = zmq_close (sock);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_capabilities (void)
{
    NSLog(@"- test_capabilities");
#if !defined (ZMQ_HAVE_WINDOWS) && !defined (ZMQ_HAVE_OPENVMS)
    assert (zmq_has ("ipc"));
#else
    assert (!zmq_has ("ipc"));
#endif
    
#if defined (ZMQ_HAVE_OPENPGM)
    assert (zmq_has ("pgm"));
#else
    assert (!zmq_has ("pgm"));
#endif
    
#if defined (ZMQ_HAVE_TIPC)
    assert (zmq_has ("tipc"));
#else
    assert (!zmq_has ("tipc"));
#endif
    
#if defined (ZMQ_HAVE_NORM)
    assert (zmq_has ("norm"));
#else
    assert (!zmq_has ("norm"));
#endif
    
#if defined (ZMQ_HAVE_CURVE)
    assert (zmq_has ("curve"));
#else
    assert (!zmq_has ("curve"));
#endif
    
#if defined (HAVE_LIBGSSAPI_KRB5)
    assert (zmq_has ("gssapi"));
#else
    assert (!zmq_has ("gssapi"));
#endif
    
#if defined (ZMQ_HAVE_VMCI)
    assert (zmq_has("vmci"));
#else
    assert (!zmq_has("vmci"));
#endif
    
#if defined (ZMQ_BUILD_DRAFT_API)
    assert (zmq_has("draft"));
#else
    assert (!zmq_has("draft"));
#endif
    return 0;
}

int test_client_server (void)
{
    NSLog(@"- test_client_server");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *server = zmq_socket (ctx, ZMQ_SERVER);
    void *client = zmq_socket (ctx, ZMQ_CLIENT);
    
    int rc = zmq_bind (server, "inproc://test-client-server");
    assert (rc == 0);
    
    rc = zmq_connect (client, "inproc://test-client-server");
    assert (rc == 0);
    
    zmq_msg_t msg;
    rc = zmq_msg_init_size (&msg, 1);
    assert (rc == 0);
    
    char *data = (char *) zmq_msg_data (&msg);
    data [0] = 1;
    
    rc = zmq_msg_send (&msg, client, ZMQ_SNDMORE);
    assert (rc == -1);
    
    rc = zmq_msg_send (&msg, client, 0);
    assert (rc == 1);
    
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    
    rc = zmq_msg_recv (&msg, server, 0);
    assert (rc == 1);
    
    uint32_t routing_id = zmq_msg_routing_id (&msg);
    assert (routing_id != 0);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    rc = zmq_msg_init_size (&msg, 1);
    assert (rc == 0);
    
    data = (char *)zmq_msg_data (&msg);
    data[0] = 2;
    
    rc = zmq_msg_set_routing_id (&msg, routing_id);
    assert (rc == 0);
    
    rc = zmq_msg_send (&msg, server, ZMQ_SNDMORE);
    assert (rc == -1);
    
    rc = zmq_msg_send (&msg, server, 0);
    assert (rc == 1);
    
    rc = zmq_msg_recv (&msg, client, 0);
    assert (rc == 1);
    
    routing_id = zmq_msg_routing_id (&msg);
    assert (routing_id == 0);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_connect_delay_tipc (void)
{
    NSLog(@"- test_connect_delay_tipc");
    int val;
    int rc;
    char buffer[16];
    // TEST 1.
    // First we're going to attempt to send messages to two
    // pipes, one connected, the other not. We should see
    // the PUSH load balancing to both pipes, and hence half
    // of the messages getting queued, as connect() creates a
    // pipe immediately.
    
    void *context = zmq_ctx_new();
    assert (context);
    void *to = zmq_socket(context, ZMQ_PULL);
    assert (to);
    
    // Bind the one valid receiver
    val = 0;
    rc = zmq_setsockopt(to, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    rc = zmq_bind (to, "tipc://{6555,0,0}");
    assert (rc == 0);
    
    // Create a socket pushing to two endpoints - only 1 message should arrive.
    void *from = zmq_socket (context, ZMQ_PUSH);
    assert(from);
    
    val = 0;
    zmq_setsockopt (from, ZMQ_LINGER, &val, sizeof (val));
    // This pipe will not connect
    rc = zmq_connect (from, "tipc://{5556,0}@0.0.0");
    assert (rc == 0);
    // This pipe will
    rc = zmq_connect (from, "tipc://{6555,0}@0.0.0");
    assert (rc == 0);
    
    // We send 10 messages, 5 should just get stuck in the queue
    // for the not-yet-connected pipe
    for (int i = 0; i < 10; ++i) {
        rc = zmq_send (from, "Hello", 5, 0);
        assert (rc == 5);
    }
    
    // We now consume from the connected pipe
    // - we should see just 5
    int timeout = 250;
    rc = zmq_setsockopt (to, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    int seen = 0;
    while (true) {
        rc = zmq_recv (to, &buffer, sizeof (buffer), 0);
        if (rc == -1)
            break;          //  Break when we didn't get a message
        seen++;
    }
    assert (seen == 5);
    
    rc = zmq_close (from);
    assert (rc == 0);
    
    rc = zmq_close (to);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    
    // TEST 2
    // This time we will do the same thing, connect two pipes,
    // one of which will succeed in connecting to a bound
    // receiver, the other of which will fail. However, we will
    // also set the delay attach on connect flag, which should
    // cause the pipe attachment to be delayed until the connection
    // succeeds.
    context = zmq_ctx_new();
    
    // Bind the valid socket
    to = zmq_socket (context, ZMQ_PULL);
    assert (to);
    rc = zmq_bind (to, "tipc://{5560,0,0}");
    assert (rc == 0);
    
    val = 0;
    rc = zmq_setsockopt (to, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    
    // Create a socket pushing to two endpoints - all messages should arrive.
    from = zmq_socket (context, ZMQ_PUSH);
    assert (from);
    
    val = 0;
    rc = zmq_setsockopt (from, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    
    // Set the key flag
    val = 1;
    rc = zmq_setsockopt (from, ZMQ_DELAY_ATTACH_ON_CONNECT, &val, sizeof(val));
    assert (rc == 0);
    
    // Connect to the invalid socket
    rc = zmq_connect (from, "tipc://{5561,0}@0.0.0");
    assert (rc == 0);
    // Connect to the valid socket
    rc = zmq_connect (from, "tipc://{5560,0}@0.0.0");
    assert (rc == 0);
    
    // Send 10 messages, all should be routed to the connected pipe
    for (int i = 0; i < 10; ++i) {
        rc = zmq_send (from, "Hello", 5, 0);
        assert (rc == 5);
    }
    rc = zmq_setsockopt (to, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    seen = 0;
    while (true) {
        rc = zmq_recv (to, &buffer, sizeof (buffer), 0);
        if (rc == -1)
            break;          //  Break when we didn't get a message
        seen++;
    }
    assert (seen == 10);
    
    rc = zmq_close (from);
    assert (rc == 0);
    
    rc = zmq_close (to);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    
    // TEST 3
    // This time we want to validate that the same blocking behaviour
    // occurs with an existing connection that is broken. We will send
    // messages to a connected pipe, disconnect and verify the messages
    // block. Then we reconnect and verify messages flow again.
    context = zmq_ctx_new ();
    
    void *backend = zmq_socket (context, ZMQ_DEALER);
    assert (backend);
    void *frontend = zmq_socket (context, ZMQ_DEALER);
    assert (frontend);
    int zero = 0;
    rc = zmq_setsockopt (backend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_setsockopt (frontend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    
    //  Frontend connects to backend using DELAY_ATTACH_ON_CONNECT
    int on = 1;
    rc = zmq_setsockopt (frontend, ZMQ_DELAY_ATTACH_ON_CONNECT, &on, sizeof (on));
    assert (rc == 0);
    rc = zmq_bind (backend, "tipc://{5560,0,0}");
    assert (rc == 0);
    rc = zmq_connect (frontend, "tipc://{5560,0}@0.0.0");
    assert (rc == 0);
    
    //  Ping backend to frontend so we know when the connection is up
    rc = zmq_send (backend, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (frontend, buffer, 255, 0);
    assert (rc == 5);
    
    // Send message from frontend to backend
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == 5);
    
    rc = zmq_close (backend);
    assert (rc == 0);
    
    //  Give time to process disconnect
    msleep (SETTLE_TIME);
    
    // Send a message, should fail
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == -1);
    
    //  Recreate backend socket
    backend = zmq_socket (context, ZMQ_DEALER);
    assert (backend);
    rc = zmq_setsockopt (backend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_bind (backend, "tipc://{5560,0,0}");
    assert (rc == 0);
    
    //  Ping backend to frontend so we know when the connection is up
    rc = zmq_send (backend, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (frontend, buffer, 255, 0);
    assert (rc == 5);
    
    // After the reconnect, should succeed
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == 5);
    
    rc = zmq_close (backend);
    assert (rc == 0);
    
    rc = zmq_close (frontend);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    return 0;
}

int test_connect_resolve (void)
{
    NSLog(@"- test_connect_resolve");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sock = zmq_socket (ctx, ZMQ_PUB);
    assert (sock);
    
    int rc = zmq_connect (sock, "tcp://localhost:1234");
    assert (rc == 0);
    
    rc = zmq_connect (sock, "tcp://[::1]:1234");
    assert (rc == 0);
    
    rc = zmq_connect (sock, "tcp://localhost:invalid");
    assert (rc == -1);
    
    rc = zmq_connect (sock, "tcp://in val id:1234");
    assert (rc == -1);
    
    rc = zmq_connect (sock, "tcp://");
    assert (rc == -1);
    
    rc = zmq_connect (sock, "tcp://192.168.0.200:*");
    assert (rc == -1);
    
    rc = zmq_connect (sock, "invalid://localhost:1234");
    assert (rc == -1);
    assert (errno == EPROTONOSUPPORT);
    
    rc = zmq_close (sock);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}



void test_stream_2_stream(){
    void *rbind, *rconn1;
    int ret;
    char buff[256];
    char msg[] = "hi 1";
    const char *bindip = "tcp://127.0.0.1:5556";
    int disabled = 0;
    int zero = 0;
    void *ctx = zmq_ctx_new ();
    
    //  Set up listener STREAM.
    rbind = zmq_socket (ctx, ZMQ_STREAM);
    assert (rbind);
    ret = zmq_setsockopt (rbind, ZMQ_STREAM_NOTIFY, &disabled, sizeof (disabled));
    assert (ret == 0);
    ret = zmq_setsockopt (rbind, ZMQ_LINGER, &zero, sizeof (zero));
    assert (0 == ret);
    ret = zmq_bind (rbind, bindip);
    assert(0 == ret);
    
    //  Set up connection stream.
    rconn1 = zmq_socket (ctx, ZMQ_STREAM);
    assert (rconn1);
    ret = zmq_setsockopt (rconn1, ZMQ_LINGER, &zero, sizeof (zero));
    assert (0 == ret);
    
    //  Do the connection.
    ret = zmq_setsockopt (rconn1, ZMQ_CONNECT_RID, "conn1", 6);
    assert (0 == ret);
    ret = zmq_connect (rconn1, bindip);
    
    /*  Uncomment to test assert on duplicate rid.
     //  Test duplicate connect attempt.
     ret = zmq_setsockopt (rconn1, ZMQ_CONNECT_RID, "conn1", 6);
     assert (0 == ret);
     ret = zmq_connect (rconn1, bindip);
     assert (0 == ret);
     */
    //  Send data to the bound stream.
    ret = zmq_send (rconn1, "conn1", 6, ZMQ_SNDMORE);
    assert (6 == ret);
    ret = zmq_send (rconn1, msg, 5, 0);
    assert (5 == ret);
    
    //  Accept data on the bound stream.
    ret = zmq_recv (rbind, buff, 256, 0);
    assert (ret);
    assert (0 == buff[0]);
    ret = zmq_recv (rbind, buff+128, 128, 0);
    assert (5 == ret);
    assert ('h' == buff[128]);
    
    // Handle close of the socket.
    ret = zmq_unbind (rbind, bindip);
    assert(0 == ret);
    ret = zmq_close (rbind);
    assert(0 == ret);
    ret = zmq_close (rconn1);
    assert(0 == ret);
    
    zmq_ctx_destroy (ctx);
}

void test_router_2_router(bool named){
    void *rbind, *rconn1;
    int ret;
    char buff[256];
    char msg[] = "hi 1";
    const char *bindip = "tcp://127.0.0.1:5556";
    int zero = 0;
    void *ctx = zmq_ctx_new ();
    
    //  Create bind socket.
    rbind = zmq_socket (ctx, ZMQ_ROUTER);
    assert (rbind);
    ret = zmq_setsockopt (rbind, ZMQ_LINGER, &zero, sizeof (zero));
    assert (0 == ret);
    ret = zmq_bind (rbind, bindip);
    assert (0 == ret);
    
    //  Create connection socket.
    rconn1 = zmq_socket (ctx, ZMQ_ROUTER);
    assert (rconn1);
    ret = zmq_setsockopt (rconn1, ZMQ_LINGER, &zero, sizeof (zero));
    assert (0 == ret);
    
    //  If we're in named mode, set some identities.
    if (named) {
        ret = zmq_setsockopt (rbind, ZMQ_IDENTITY, "X", 1);
        ret = zmq_setsockopt (rconn1, ZMQ_IDENTITY, "Y", 1);
    }
    
    //  Make call to connect using a connect_rid.
    ret = zmq_setsockopt (rconn1, ZMQ_CONNECT_RID, "conn1", 6);
    assert (0 == ret);
    ret = zmq_connect (rconn1, bindip);
    assert (0 == ret);
    /*  Uncomment to test assert on duplicate rid
     //  Test duplicate connect attempt.
     ret = zmq_setsockopt (rconn1, ZMQ_CONNECT_RID, "conn1", 6);
     assert (0 == ret);
     ret = zmq_connect (rconn1, bindip);
     assert (0 == ret);
     */
    //  Send some data.
    ret = zmq_send (rconn1, "conn1", 6, ZMQ_SNDMORE);
    assert (6 == ret);
    ret = zmq_send (rconn1, msg, 5, 0);
    assert (5 == ret);
    
    //  Receive the name.
    ret = zmq_recv (rbind, buff, 256, 0);
    if (named)
        assert (ret && 'Y' == buff[0]);
    else
        assert (ret && 0 == buff[0]);
    
    //  Receive the data.
    ret = zmq_recv (rbind, buff+128, 128, 0);
    assert(5 == ret && 'h' == buff[128]);
    
    //  Send some data back.
    if (named) {
        ret = zmq_send (rbind, buff, 1, ZMQ_SNDMORE);
        assert (1 == ret);
    }
    else {
        ret = zmq_send (rbind, buff, 5, ZMQ_SNDMORE);
        assert (5 == ret);
    }
    ret = zmq_send_const (rbind, "ok", 3, 0);
    assert (3 == ret);
    
    //  If bound socket identity naming a problem, we'll likely see something funky here.
    ret = zmq_recv (rconn1, buff, 256, 0);
    assert ('c' == buff[0] && 6 == ret);
    ret = zmq_recv (rconn1, buff+128, 128, 0);
    assert (3 == ret && 'o' == buff[128]);
    
    ret = zmq_unbind (rbind, bindip);
    assert(0 == ret);
    ret = zmq_close (rbind);
    assert(0 == ret);
    ret = zmq_close (rconn1);
    assert(0 == ret);
    
    zmq_ctx_destroy (ctx);
}

int test_connect_rid (void)
{
    NSLog(@"- test_connect_rid");
    setup_test_environment ();
    
    test_stream_2_stream ();
    test_router_2_router (false);
    test_router_2_router (true);
    
    return 0;
}

static void receiver (void *socket)
{
    char buffer[16];
    int rc = zmq_recv (socket, &buffer, sizeof (buffer), 0);
    assert(rc == -1);
}

void test_ctx_destroy1()
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *socket = zmq_socket (ctx, ZMQ_PULL);
    assert (socket);
    
    // Close the socket
    rc = zmq_close (socket);
    assert (rc == 0);
    
    // Test error - API has multiple ways to kill Contexts
    rc = zmq_ctx_term (NULL);
    assert (rc == -1 && errno == EFAULT);
    rc = zmq_term (NULL);
    assert (rc == -1 && errno == EFAULT);
    
    // Destroy the context
    rc = zmq_ctx_destroy (ctx);
    assert (rc == 0);
}

void test_ctx_shutdown1()
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *socket = zmq_socket (ctx, ZMQ_PULL);
    assert (socket);
    
    // Spawn a thread to receive on socket
    void *receiver_thread = zmq_threadstart (&receiver, socket);
    
    // Wait for thread to start up and block
    msleep (SETTLE_TIME);
    
    // Test error - Shutdown context
    rc = zmq_ctx_shutdown (NULL);
    assert (rc == -1 && errno == EFAULT);
    
    // Shutdown context, if we used destroy here we would deadlock.
    rc = zmq_ctx_shutdown (ctx);
    assert (rc == 0);
    
    // Wait for thread to finish
    zmq_threadclose (receiver_thread);
    
    // Close the socket.
    rc = zmq_close (socket);
    assert (rc == 0);
    
    // Destory the context, will now not hang as we have closed the socket.
    rc = zmq_ctx_destroy (ctx);
    assert (rc == 0);
}

int test_ctx_destroy (void)
{
    NSLog(@"- test_ctx_destroy");
    setup_test_environment();
    
    test_ctx_destroy1();
    test_ctx_shutdown1();
    
    return 0;
}

int test_ctx_options (void)
{
    NSLog(@"- test_ctx_options");
    setup_test_environment();
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    assert (zmq_ctx_get (ctx, ZMQ_MAX_SOCKETS) == ZMQ_MAX_SOCKETS_DFLT);
#if defined(ZMQ_USE_SELECT)
    assert (zmq_ctx_get (ctx, ZMQ_SOCKET_LIMIT) == FD_SETSIZE - 1);
#elif    defined(ZMQ_USE_POLL) || defined(ZMQ_USE_EPOLL)     \
|| defined(ZMQ_USE_DEVPOLL) || defined(ZMQ_USE_KQUEUE)
    assert (zmq_ctx_get (ctx, ZMQ_SOCKET_LIMIT) == 65535);
#endif
    assert (zmq_ctx_get (ctx, ZMQ_IO_THREADS) == ZMQ_IO_THREADS_DFLT);
    assert (zmq_ctx_get (ctx, ZMQ_IPV6) == 0);
#if defined (ZMQ_BUILD_DRAFT_AP)
    assert (zmq_ctx_get (ctx, ZMQ_MSG_T_SIZE) == sizeof (zmq_msg_t));
#endif
    
    rc = zmq_ctx_set (ctx, ZMQ_IPV6, true);
    assert (zmq_ctx_get (ctx, ZMQ_IPV6) == 1);
    
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    int value;
    size_t optsize = sizeof (int);
    rc = zmq_getsockopt (router, ZMQ_IPV6, &value, &optsize);
    assert (rc == 0);
    assert (value == 1);
    rc = zmq_getsockopt (router, ZMQ_LINGER, &value, &optsize);
    assert (rc == 0);
    assert (value == -1);
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_ctx_set (ctx, ZMQ_BLOCKY, false);
    assert (zmq_ctx_get (ctx, ZMQ_BLOCKY) == 0);
    router = zmq_socket (ctx, ZMQ_ROUTER);
    rc = zmq_getsockopt (router, ZMQ_LINGER, &value, &optsize);
    assert (rc == 0);
    assert (value == 0);
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

void str_send_to (void *s_, const char *content_, const char *address_)
{
    //  Send the address part
    int rc = s_sendmore (s_, address_);
    assert (rc > 0);
    
    rc = s_send (s_, content_);
    assert (rc > 0);
}

void str_recv_from (void *s_, char **ptr_content_, char **ptr_address_)
{
    *ptr_address_ = s_recv (s_);
    assert (ptr_address_);
    
    *ptr_content_ = s_recv (s_);
    assert (ptr_content_);
}

int test_dgram(void)
{
    NSLog(@"- test_dgram");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    char* message_string;
    char* address;
    
    void *sender = zmq_socket (ctx, ZMQ_DGRAM);
    void *listener = zmq_socket (ctx, ZMQ_DGRAM);
    
    //  Connecting dgram shoudl fail
    int rc = zmq_connect (listener, "udp://127.0.0.1:5556");
    assert (rc == -1);
    
    rc = zmq_bind (listener, "udp://*:5556");
    assert (rc == 0);
    
    rc = zmq_bind (sender, "udp://*:5557");
    assert (rc == 0);
    
    str_send_to (sender, "Is someone there ?", "127.0.0.1:5556");
    
    str_recv_from (listener, &message_string, &address);
    assert (strcmp(message_string, "Is someone there ?") == 0);
    assert (strcmp(address, "127.0.0.1:5557") == 0);
    free (message_string);
    
    str_send_to (listener, "Yes, there is !", address);
    free (address);
    
    str_recv_from (sender, &message_string, &address);
    assert (strcmp(message_string, "Yes, there is !") == 0);
    assert (strcmp(address, "127.0.0.1:5556") == 0);
    free (message_string);
    free (address);
    
    rc = zmq_close (sender);
    assert (rc == 0);
    
    rc = zmq_close (listener);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_diffserver (void)
{
    NSLog(@"- test_diffserver");
    int rc;
    int tos = 0x28;
    int o_tos;
    size_t tos_size = sizeof(tos);
    
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    rc = zmq_setsockopt (sb, ZMQ_TOS, &tos, tos_size);
    assert (rc == 0);
    rc = zmq_bind (sb, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_getsockopt (sb, ZMQ_TOS, &o_tos, &tos_size);
    assert (rc == 0);
    assert (o_tos == tos);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    tos = 0x58;
    rc = zmq_setsockopt (sc, ZMQ_TOS, &tos, tos_size);
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_getsockopt (sc, ZMQ_TOS, &o_tos, &tos_size);
    assert (rc == 0);
    assert (o_tos == tos);
    
    // Wireshark can be used to verify that the server socket is
    // using DSCP 0x28 in packets to the client while the client
    // is using 0x58 in packets to the server.
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
    
}


/// Initialize a zeromq message with a given null-terminated string
#define ZMQ_PREPARE_STRING(msg, data, size) \
zmq_msg_init(&msg) && printf("zmq_msg_init: %s\n", zmq_strerror(errno)); \
zmq_msg_init_size (&msg, size + 1) && printf("zmq_msg_init_size: %s\n",zmq_strerror(errno)); \
memcpy(zmq_msg_data(&msg), data, size + 1);

//  TODO: this code fails to meet our style guidelines, and needs rewriting

static int publicationsReceived = 0;
static bool isSubscribed = false;

int test_disconnect_inproc(void)
{
    NSLog(@"- test_disconnect_inproc");
    setup_test_environment();
    void* context = zmq_ctx_new();
    void* pubSocket;
    void* subSocket;
    
    (pubSocket = zmq_socket(context, ZMQ_XPUB))         || printf("zmq_socket: %s\n", zmq_strerror(errno));
    (subSocket = zmq_socket(context, ZMQ_SUB))          || printf("zmq_socket: %s\n", zmq_strerror(errno));
    zmq_setsockopt(subSocket, ZMQ_SUBSCRIBE, "foo", 3)  && printf("zmq_setsockopt: %s\n",zmq_strerror(errno));
    
    zmq_bind(pubSocket, "inproc://someInProcDescriptor") && printf("zmq_bind: %s\n", zmq_strerror(errno));
    //zmq_bind(pubSocket, "tcp://127.0.0.1:30010") && printf("zmq_bind: %s\n", zmq_strerror(errno));
    
    int more;
    size_t more_size = sizeof(more);
    int iteration = 0;
    
    while (1) {
        zmq_pollitem_t items [] = {
            { subSocket, 0, ZMQ_POLLIN, 0 }, // read publications
            { pubSocket, 0, ZMQ_POLLIN, 0 }, // read subscriptions
        };
        int rc = zmq_poll (items, 2, 100);
        
        if (items [1].revents & ZMQ_POLLIN) {
            while (1) {
                zmq_msg_t msg;
                zmq_msg_init (&msg);
                zmq_msg_recv (&msg, pubSocket, 0);
                char* buffer = (char*)zmq_msg_data(&msg);
                
                if (buffer[0] == 0) {
                    assert(isSubscribed);
                    isSubscribed = false;
                }
                else {
                    assert(!isSubscribed);
                    isSubscribed = true;
                }
                
                zmq_getsockopt (pubSocket, ZMQ_RCVMORE, &more, &more_size);
                zmq_msg_close (&msg);
                
                if (!more)
                    break;      //  Last message part
            }
        }
        
        if (items[0].revents & ZMQ_POLLIN) {
            while (1) {
                zmq_msg_t msg;
                zmq_msg_init (&msg);
                zmq_msg_recv (&msg, subSocket, 0);
                zmq_getsockopt (subSocket, ZMQ_RCVMORE, &more, &more_size);
                zmq_msg_close (&msg);
                
                if (!more) {
                    publicationsReceived++;
                    break;      //  Last message part
                }
            }
        }
        if (iteration == 1) {
            zmq_connect(subSocket, "inproc://someInProcDescriptor") && printf("zmq_connect: %s\n", zmq_strerror(errno));
            msleep (SETTLE_TIME);
        }
        if (iteration == 4) {
            zmq_disconnect(subSocket, "inproc://someInProcDescriptor") && printf("zmq_disconnect(%d): %s\n", errno, zmq_strerror(errno));
        }
        if (iteration > 4 && rc == 0)
            break;
        
        zmq_msg_t channelEnvlp;
        ZMQ_PREPARE_STRING(channelEnvlp, "foo", 3);
        zmq_msg_send (&channelEnvlp, pubSocket, ZMQ_SNDMORE) >= 0 || printf("zmq_msg_send: %s\n",zmq_strerror(errno));
        zmq_msg_close(&channelEnvlp) && printf("zmq_msg_close: %s\n",zmq_strerror(errno));
        
        zmq_msg_t message;
        ZMQ_PREPARE_STRING(message, "this is foo!", 12);
        zmq_msg_send (&message, pubSocket, 0) >= 0 || printf("zmq_msg_send: %s\n",zmq_strerror(errno));
        zmq_msg_close(&message) && printf("zmq_msg_close: %s\n",zmq_strerror(errno));
        
        iteration++;
    }
    assert(publicationsReceived == 3);
    assert(!isSubscribed);
    
    zmq_close(pubSocket) && printf("zmq_close: %s", zmq_strerror(errno));
    zmq_close(subSocket) && printf("zmq_close: %s", zmq_strerror(errno));
    
    zmq_ctx_term(context);
    return 0;
}

static void bounce_fail (void *server, void *client)
{
    const char *content = "12345678ABCDEFGH12345678abcdefgh";
    char buffer [32];
    
    //  Send message from client to server
    int rc = zmq_send (client, content, 32, ZMQ_SNDMORE);
    assert (rc == 32);
    rc = zmq_send (client, content, 32, 0);
    assert (rc == 32);
    
    //  Receive message at server side (should not succeed)
    int timeout = 250;
    rc = zmq_setsockopt (server, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_recv (server, buffer, 32, 0);
    assert (rc == -1);
    assert (zmq_errno () == EAGAIN);
    
    //  Send message from server to client to test other direction
    rc = zmq_setsockopt (server, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_send (server, content, 32, ZMQ_SNDMORE);
    assert (rc == -1);
    assert (zmq_errno () == EAGAIN);
}

template <class T>
static void run_test (int opt, T optval, int expected_error, int bounce_test)
{
    int rc;
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_DEALER);
    assert (sb);
    
    if (opt) {
        rc = zmq_setsockopt(sb, opt, &optval, sizeof (optval));
        if (expected_error) {
            assert (rc == -1);
            assert (zmq_errno () == expected_error);
        }
        else
            assert (rc == 0);
    }
    
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    
    // If a test fails, don't hang for too long
    int timeout = 2500;
    rc = zmq_setsockopt (sb, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sb, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sc, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sc, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    int interval = -1;
    rc = zmq_setsockopt (sc, ZMQ_RECONNECT_IVL, &interval, sizeof (int));
    assert (rc == 0);
    
    if (bounce_test) {
        const char* endpoint = "ipc://test_filter_ipc.sock";
        int rc = zmq_bind (sb, endpoint);
        assert (rc == 0);
        
        rc = zmq_connect (sc, endpoint);
        assert (rc == 0);
        
        if (bounce_test > 0)
            bounce (sb, sc);
        else
            bounce_fail (sb, sc);
    }
    close_zero_linger (sc);
    close_zero_linger (sb);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

int test_filter_inproc (void)
{
    NSLog(@"- test_filter_inproc");
#if !defined (ZMQ_HAVE_WINDOWS)
    setup_test_environment();
    
    // No filters
    run_test<int> (0, 0, 0, 1);
    
#if defined ZMQ_HAVE_SO_PEERCRED || defined ZMQ_HAVE_LOCAL_PEERCRED
    // Get the group and supplimental groups of the process owner
    gid_t groups[100];
    int ngroups = getgroups(100, groups);
    assert (ngroups != -1);
    gid_t group = getgid(), supgroup = group, notgroup = group + 1;
    for (int i = 0; i < ngroups; i++) {
        if (supgroup == group && group != groups[i])
            supgroup = groups[i];
        if (notgroup <= groups[i])
            notgroup = groups[i] + 1;
    }
    
    // Test filter with UID of process owner
    run_test<uid_t> (ZMQ_IPC_FILTER_UID, getuid(), 0, 1);
    // Test filter with UID of another (possibly non-existent) user
    run_test<uid_t> (ZMQ_IPC_FILTER_UID, getuid() + 1, 0, -1);
    // Test filter with GID of process owner
    run_test<gid_t> (ZMQ_IPC_FILTER_GID, group, 0, 1);
    // Test filter with supplimental group of process owner
    run_test<gid_t> (ZMQ_IPC_FILTER_GID, supgroup, 0, 1);
    // Test filter with GID of another (possibly non-existent) group
    run_test<gid_t> (ZMQ_IPC_FILTER_GID, notgroup, 0, -1);
#   if defined ZMQ_HAVE_SO_PEERCRED
    // Test filter with PID of current process
    run_test<pid_t> (ZMQ_IPC_FILTER_PID, getpid(), 0, 1);
    // Test filter with PID of another (possibly non-existent) process
    run_test<pid_t> (ZMQ_IPC_FILTER_PID, getpid() + 1, 0, -1);
#   else
    // Setup of PID filter should fail with operation not supported error
    run_test<pid_t> (ZMQ_IPC_FILTER_PID, getpid(), EINVAL, 0);
#   endif
#else
    run_test<uid_t> (ZMQ_IPC_FILTER_UID, 0, EINVAL, 0);
    run_test<gid_t> (ZMQ_IPC_FILTER_GID, 0, EINVAL, 0);
    run_test<pid_t> (ZMQ_IPC_FILTER_PID, 0, EINVAL, 0);
#endif // defined ZMQ_HAVE_SO_PEERCRED || defined ZMQ_HAVE_LOCAL_PEERCRED
    
#endif
    return 0 ;
}


const char *address = "tcp://127.0.0.1:6571";

#define NUM_MESSAGES 5

int test_fork (void)
{
    NSLog(@"- test_fork");
#if !defined (ZMQ_HAVE_WINDOWS)
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create and bind pull socket to receive messages
    void *pull = zmq_socket (ctx, ZMQ_PULL);
    assert (pull);
    int rc = zmq_bind (pull, address);
    assert (rc == 0);
    
    int pid = fork ();
    if (pid == 0) {
        //  Child process
        //  Immediately close parent sockets and context
        zmq_close (pull);
        zmq_ctx_term (ctx);
        
        //  Create new context, socket, connect and send some messages
        void *child_ctx = zmq_ctx_new ();
        assert (child_ctx);
        void *push = zmq_socket (child_ctx, ZMQ_PUSH);
        assert (push);
        rc = zmq_connect (push, address);
        assert (rc == 0);
        int count;
        for (count = 0; count < NUM_MESSAGES; count++)
            zmq_send (push, "Hello", 5, 0);
        
        zmq_close (push);
        zmq_ctx_destroy (child_ctx);
        exit (0);
    }
    else {
        //  Parent process
        int count;
        for (count = 0; count < NUM_MESSAGES; count++) {
            char buffer [5];
            int num_bytes = zmq_recv (pull, buffer, 5, 0);
            assert (num_bytes == 5);
        }
        int child_status;
        while (true) {
            rc = waitpid (pid, &child_status, 0);
            if (rc == -1 && errno == EINTR)
                continue;
            assert (rc > 0);
            //  Verify the status code of the child was zero
            assert (WEXITSTATUS (child_status) == 0);
            break;
        }
        zmq_close (pull);
        zmq_ctx_term (ctx);
        exit (0);
    }
#endif
    return 0;
}

int test_getsockopt_memset (void)
{
    NSLog(@"- test_getsockopt_memset");
    int64_t more;
    size_t more_size = sizeof(more);
    
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PUB);
    assert (sb);
    int rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_SUB);
    assert (sc);
    rc = zmq_connect (sc, "inproc://a");
    assert (rc == 0);
    
    memset(&more, 0xFF, sizeof(int64_t));
    zmq_getsockopt(sc, ZMQ_RCVMORE, &more, &more_size);
    assert (more_size == sizeof(int));
    assert (more == 0);
    
    
    // Cleanup
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

//  Read one event off the monitor socket; return value and address
//  by reference, if not null, and event number by value. Returns -1
//  in case of error.

static int
hearbeat_get_monitor_event (void *monitor)
{
    for (int i = 0; i < 2; i++) {
        //  First frame in message contains event number and value
        zmq_msg_t msg;
        int rc = zmq_msg_init (&msg);
        assert (rc == 0);
        if (zmq_msg_recv (&msg, monitor, ZMQ_DONTWAIT) == -1) {
            msleep (SETTLE_TIME);
            continue;           //  Interruped, presumably
        }
        assert (zmq_msg_more (&msg));
        
        uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
        uint16_t event = *(uint16_t *) (data);
        
        //  Second frame in message contains event address
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        if (zmq_msg_recv (&msg, monitor, 0) == -1) {
            return -1;              //  Interruped, presumably
        }
        assert (!zmq_msg_more (&msg));
        
        return event;
    }
    return -1;
}

static void
hearbeat_recv_with_retry (raw_socket fd, char *buffer, int bytes) {
    int received = 0;
    while (true) {
        int rc = recv(fd, buffer + received, bytes - received, 0);
        assert(rc > 0);
        received += rc;
        assert(received <= bytes);
        if (received == bytes) break;
    }
}

static void
hearbeat_mock_handshake (raw_socket fd) {
    const uint8_t zmtp_greeting[33] = { 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0x7f, 3, 0, 'N', 'U', 'L', 'L', 0 };
    char buffer [128];
    memset (buffer, 0, sizeof(buffer));
    memcpy (buffer, zmtp_greeting, sizeof(zmtp_greeting));
    
    int rc = send (fd, buffer, 64, 0);
    assert (rc == 64);
    
    hearbeat_recv_with_retry (fd, buffer, 64);
    
    const uint8_t zmtp_ready [43] = {
        4, 41, 5, 'R', 'E', 'A', 'D', 'Y', 11, 'S', 'o', 'c', 'k', 'e', 't', '-', 'T', 'y', 'p', 'e',
        0, 0, 0, 6, 'D', 'E', 'A', 'L', 'E', 'R', 8, 'I', 'd', 'e', 'n', 't', 'i', 't', 'y',
        0, 0, 0, 0
    };
    
    memset(buffer, 0, sizeof(buffer));
    memcpy(buffer, zmtp_ready, 43);
    rc = send(fd, buffer, 43, 0);
    assert (rc == 43);
    
    hearbeat_recv_with_retry(fd, buffer, 43);
}

static void
hearbeat_prep_server_socket(void * ctx, int set_heartbeats, void ** server_out, void ** mon_out)
{
    int rc;
    //  We'll be using this socket in raw mode
    void *server = zmq_socket (ctx, ZMQ_ROUTER);
    assert (server);
    
    int value = 0;
    rc = zmq_setsockopt (server, ZMQ_LINGER, &value, sizeof (value));
    assert (rc == 0);
    
    if (set_heartbeats) {
        value = 50;
        rc = zmq_setsockopt (server, ZMQ_HEARTBEAT_IVL, &value, sizeof(value));
        assert (rc == 0);
    }
    
    rc = zmq_bind (server, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    //  Create and connect a socket for collecting monitor events on dealer
    void *server_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (server_mon);
    
    rc = zmq_socket_monitor (server, "inproc://monitor-dealer",
                             ZMQ_EVENT_CONNECTED | ZMQ_EVENT_DISCONNECTED | ZMQ_EVENT_ACCEPTED);
    assert (rc == 0);
    
    //  Connect to the inproc endpoint so we'll get events
    rc = zmq_connect (server_mon, "inproc://monitor-dealer");
    assert (rc == 0);
    
    *server_out = server;
    *mon_out = server_mon;
}

// This checks for a broken TCP connection (or, in this case a stuck one
// where the peer never responds to PINGS). There should be an accepted event
// then a disconnect event.
static void
test_heartbeat_timeout (void)
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void * server, * server_mon;
    hearbeat_prep_server_socket (ctx, 1, &server, &server_mon);
    
    struct sockaddr_in ip4addr;
    raw_socket s;
    
    ip4addr.sin_family = AF_INET;
    ip4addr.sin_port = htons (5556);
#if defined (ZMQ_HAVE_WINDOWS) && (_WIN32_WINNT < 0x0600)
    ip4addr.sin_addr.s_addr = inet_addr ("127.0.0.1");
#else
    inet_pton(AF_INET, "127.0.0.1", &ip4addr.sin_addr);
#endif
    
    s = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
    rc = connect (s, (struct sockaddr*) &ip4addr, sizeof ip4addr);
    assert (rc > -1);
    
    // Mock a ZMTP 3 client so we can forcibly time out a connection
    hearbeat_mock_handshake (s);
    
    // By now everything should report as connected
    rc = hearbeat_get_monitor_event(server_mon);
    assert (rc == ZMQ_EVENT_ACCEPTED);
    
    // We should have been disconnected
    rc = hearbeat_get_monitor_event(server_mon);
    assert (rc == ZMQ_EVENT_DISCONNECTED);
    
    close(s);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (server_mon);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

// This checks that peers respect the TTL value in ping messages
// We set up a mock ZMTP 3 client and send a ping message with a TLL
// to a server that is not doing any heartbeating. Then we sleep,
// if the server disconnects the client, then we know the TTL did
// its thing correctly.
static void
test_heartbeat_ttl (void)
{
    int rc, value;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void * server, * server_mon, *client;
    hearbeat_prep_server_socket (ctx, 0, &server, &server_mon);
    
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client != NULL);
    
    // Set the heartbeat TTL to 0.1 seconds
    value = 100;
    rc = zmq_setsockopt (client, ZMQ_HEARTBEAT_TTL, &value, sizeof (value));
    assert (rc == 0);
    
    // Set the heartbeat interval to much longer than the TTL so that
    // the socket times out oon the remote side.
    value = 250;
    rc = zmq_setsockopt (client, ZMQ_HEARTBEAT_IVL, &value, sizeof (value));
    assert (rc == 0);
    
    rc = zmq_connect (client, "tcp://localhost:5556");
    assert (rc == 0);
    
    // By now everything should report as connected
    rc = hearbeat_get_monitor_event (server_mon);
    assert (rc == ZMQ_EVENT_ACCEPTED);
    
    msleep (SETTLE_TIME);
    
    // We should have been disconnected
    rc = hearbeat_get_monitor_event (server_mon);
    assert (rc == ZMQ_EVENT_DISCONNECTED);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (server_mon);
    assert (rc == 0);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

// This checks for normal operation - that is pings and pongs being
// exchanged normally. There should be an accepted event on the server,
// and then no event afterwards.
static void
test_heartbeat_notimeout (void)
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void * server, * server_mon;
    hearbeat_prep_server_socket(ctx, 1, &server, &server_mon);
    
    void * client = zmq_socket (ctx, ZMQ_DEALER);
    rc = zmq_connect (client, "tcp://127.0.0.1:5556");
    
    // Give it a sec to connect and handshake
    msleep (SETTLE_TIME);
    
    // By now everything should report as connected
    rc = hearbeat_get_monitor_event(server_mon);
    assert (rc == ZMQ_EVENT_ACCEPTED);
    
    // We should still be connected because pings and pongs are happenin'
    rc = hearbeat_get_monitor_event (server_mon);
    assert (rc == -1);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (server_mon);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_heartbeats (void)
{
    NSLog(@"- test_heartbeats");
    setup_test_environment ();
    test_heartbeat_timeout ();
    test_heartbeat_ttl ();
    test_heartbeat_notimeout ();
}


const int MAX_SENDS = 10000;

enum TestType { BIND_FIRST, CONNECT_FIRST };

int test_defaults ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up bind socket
    void *bind_socket = zmq_socket (ctx, ZMQ_PULL);
    assert (bind_socket);
    rc = zmq_bind (bind_socket, "inproc://a");
    assert (rc == 0);
    
    // Set up connect socket
    void *connect_socket = zmq_socket (ctx, ZMQ_PUSH);
    assert (connect_socket);
    rc = zmq_connect (connect_socket, "inproc://a");
    assert (rc == 0);
    
    // Send until we block
    int send_count = 0;
    while (send_count < MAX_SENDS && zmq_send (connect_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    // Now receive all sent messages
    int recv_count = 0;
    while (zmq_recv (bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++recv_count;
    
    assert (send_count == recv_count);
    
    // Clean up
    rc = zmq_close (connect_socket);
    assert (rc == 0);
    
    rc = zmq_close (bind_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return send_count;
}

int count_msg (int send_hwm, int recv_hwm, TestType testType)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    void *bind_socket;
    void *connect_socket;
    if (testType == BIND_FIRST)
    {
        // Set up bind socket
        bind_socket = zmq_socket (ctx, ZMQ_PULL);
        assert (bind_socket);
        rc = zmq_setsockopt (bind_socket, ZMQ_RCVHWM, &recv_hwm, sizeof (recv_hwm));
        assert (rc == 0);
        rc = zmq_bind (bind_socket, "inproc://a");
        assert (rc == 0);
        
        // Set up connect socket
        connect_socket = zmq_socket (ctx, ZMQ_PUSH);
        assert (connect_socket);
        rc = zmq_setsockopt (connect_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
        assert (rc == 0);
        rc = zmq_connect (connect_socket, "inproc://a");
        assert (rc == 0);
    }
    else
    {
        // Set up connect socket
        connect_socket = zmq_socket (ctx, ZMQ_PUSH);
        assert (connect_socket);
        rc = zmq_setsockopt (connect_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
        assert (rc == 0);
        rc = zmq_connect (connect_socket, "inproc://a");
        assert (rc == 0);
        
        // Set up bind socket
        bind_socket = zmq_socket (ctx, ZMQ_PULL);
        assert (bind_socket);
        rc = zmq_setsockopt (bind_socket, ZMQ_RCVHWM, &recv_hwm, sizeof (recv_hwm));
        assert (rc == 0);
        rc = zmq_bind (bind_socket, "inproc://a");
        assert (rc == 0);
    }
    
    // Send until we block
    int send_count = 0;
    while (send_count < MAX_SENDS && zmq_send (connect_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    // Now receive all sent messages
    int recv_count = 0;
    while (zmq_recv (bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++recv_count;
    
    assert (send_count == recv_count);
    
    // Now it should be possible to send one more.
    rc = zmq_send (connect_socket, NULL, 0, 0);
    assert (rc == 0);
    
    //  Consume the remaining message.
    rc = zmq_recv (bind_socket, NULL, 0, 0);
    assert (rc == 0);
    
    // Clean up
    rc = zmq_close (connect_socket);
    assert (rc == 0);
    
    rc = zmq_close (bind_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return send_count;
}

int test_inproc_bind_first (int send_hwm, int recv_hwm)
{
    return count_msg(send_hwm, recv_hwm, BIND_FIRST);
}

int test_inproc_connect_first (int send_hwm, int recv_hwm)
{
    return count_msg(send_hwm, recv_hwm, CONNECT_FIRST);
}

int test_inproc_connect_and_close_first (int send_hwm, int recv_hwm)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up connect socket
    void *connect_socket = zmq_socket (ctx, ZMQ_PUSH);
    assert (connect_socket);
    rc = zmq_setsockopt (connect_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
    assert (rc == 0);
    rc = zmq_connect (connect_socket, "inproc://a");
    assert (rc == 0);
    
    // Send until we block
    int send_count = 0;
    while (send_count < MAX_SENDS && zmq_send (connect_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    // Close connect
    rc = zmq_close (connect_socket);
    assert (rc == 0);
    
    // Set up bind socket
    void *bind_socket = zmq_socket (ctx, ZMQ_PULL);
    assert (bind_socket);
    rc = zmq_setsockopt (bind_socket, ZMQ_RCVHWM, &recv_hwm, sizeof (recv_hwm));
    assert (rc == 0);
    rc = zmq_bind (bind_socket, "inproc://a");
    assert (rc == 0);
    
    // Now receive all sent messages
    int recv_count = 0;
    while (zmq_recv (bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++recv_count;
    
    assert (send_count == recv_count);
    
    // Clean up
    rc = zmq_close (bind_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return send_count;
}

int test_inproc_bind_and_close_first (int send_hwm, int /* recv_hwm */)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up bind socket
    void *bind_socket = zmq_socket (ctx, ZMQ_PUSH);
    assert (bind_socket);
    rc = zmq_setsockopt (bind_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
    assert (rc == 0);
    rc = zmq_bind (bind_socket, "inproc://a");
    assert (rc == 0);
    
    // Send until we block
    int send_count = 0;
    while (send_count < MAX_SENDS && zmq_send (bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    // Close bind
    rc = zmq_close (bind_socket);
    assert (rc == 0);
    
    /* Can't currently do connect without then wiring up a bind as things hang, this needs top be fixed.
     // Set up connect socket
     void *connect_socket = zmq_socket (ctx, ZMQ_PULL);
     assert (connect_socket);
     rc = zmq_setsockopt (connect_socket, ZMQ_RCVHWM, &recv_hwm, sizeof (recv_hwm));
     assert (rc == 0);
     rc = zmq_connect (connect_socket, "inproc://a");
     assert (rc == 0);
     
     // Now receive all sent messages
     int recv_count = 0;
     while (zmq_recv (connect_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
     ++recv_count;
     
     assert (send_count == recv_count);
     */
    
    // Clean up
    //rc = zmq_close (connect_socket);
    //assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return send_count;
}

int test_hwm (void)
{
    NSLog(@"- test_hwm");
    setup_test_environment();
    
    int count;
    
    // Default values are 1000 on send and 1000 one receive, so 2000 total
    count = test_defaults ();
    assert (count == 2000);
    
    // Infinite send and receive buffer
    count = test_inproc_bind_first (0, 0);
    assert (count == MAX_SENDS);
    count = test_inproc_connect_first (0, 0);
    assert (count == MAX_SENDS);
    
    // Infinite receive buffer
    count = test_inproc_bind_first (1, 0);
    assert (count == MAX_SENDS);
    count = test_inproc_connect_first (1, 0);
    assert (count == MAX_SENDS);
    
    // Infinite send buffer
    count = test_inproc_bind_first (0, 1);
    assert (count == MAX_SENDS);
    count = test_inproc_connect_first (0, 1);
    assert (count == MAX_SENDS);
    
    // Send and recv buffers hwm 1, so total that can be queued is 2
    count = test_inproc_bind_first (1, 1);
    assert (count == 2);
    count = test_inproc_connect_first (1, 1);
    assert (count == 2);
    
    // Send hwm of 1, send before bind so total that can be queued is 1
    count = test_inproc_connect_and_close_first (1, 0);
    assert (count == 1);
    
    // Send hwm of 1, send from bind side before connect so total that can be queued should be 1,
    // however currently all messages get thrown away before the connect.  BUG?
    count = test_inproc_bind_and_close_first (1, 0);
    //assert (count == 1);
    
    return 0;
}

int test_defaults (int send_hwm, int msgCnt)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up bind socket
    void *pub_socket = zmq_socket (ctx, ZMQ_PUB);
    assert (pub_socket);
    rc = zmq_bind (pub_socket, "inproc://a");
    assert (rc == 0);
    
    // Set up connect socket
    void *sub_socket = zmq_socket (ctx, ZMQ_SUB);
    assert (sub_socket);
    rc = zmq_connect (sub_socket, "inproc://a");
    assert (rc == 0);
    
    //set a hwm on publisher
    rc = zmq_setsockopt (pub_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
    rc = zmq_setsockopt( sub_socket, ZMQ_SUBSCRIBE, 0, 0);
    
    // Send until we block
    int send_count = 0;
    while (send_count < msgCnt && zmq_send (pub_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    // Now receive all sent messages
    int recv_count = 0;
    while (0 == zmq_recv (sub_socket, NULL, 0, ZMQ_DONTWAIT))
    {
        ++recv_count;
    }
    
    assert (send_hwm == recv_count);
    
    // Clean up
    rc = zmq_close (sub_socket);
    assert (rc == 0);
    
    rc = zmq_close (pub_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return recv_count;
}

int receive( void* socket)
{
    int recv_count = 0;
    // Now receive all sent messages
    while (0 == zmq_recv (socket, NULL, 0, ZMQ_DONTWAIT))
    {
        ++recv_count;
    }
    
    return recv_count;
    
}


int test_blocking (int send_hwm, int msgCnt)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up bind socket
    void *pub_socket = zmq_socket (ctx, ZMQ_PUB);
    assert (pub_socket);
    rc = zmq_bind (pub_socket, "inproc://a");
    assert (rc == 0);
    
    // Set up connect socket
    void *sub_socket = zmq_socket (ctx, ZMQ_SUB);
    assert (sub_socket);
    rc = zmq_connect (sub_socket, "inproc://a");
    assert (rc == 0);
    
    //set a hwm on publisher
    rc = zmq_setsockopt (pub_socket, ZMQ_SNDHWM, &send_hwm, sizeof (send_hwm));
    int wait = 1;
    rc = zmq_setsockopt (pub_socket, ZMQ_XPUB_NODROP, &wait, sizeof(wait));
    rc = zmq_setsockopt( sub_socket, ZMQ_SUBSCRIBE, 0, 0);
    
    // Send until we block
    int send_count = 0;
    int recv_count = 0;
    while (send_count < msgCnt )
    {
        rc = zmq_send (pub_socket, NULL, 0, ZMQ_DONTWAIT);
        if( rc == 0)
        {
            ++send_count;
        }
        else if( -1 == rc)
        {
            assert(EAGAIN == errno);
            recv_count += receive(sub_socket);
            assert(recv_count == send_count);
        }
    }
    
    recv_count += receive(sub_socket);
    
    // Clean up
    rc = zmq_close (sub_socket);
    assert (rc == 0);
    
    rc = zmq_close (pub_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return recv_count;
}

// with hwm 11024: send 9999 msg, receive 9999, send 1100, receive 1100
void test_reset_hwm ()
{
    int first_count = 9999;
    int second_count = 1100;
    int hwm = 11024;
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    // Set up bind socket
    void *pub_socket = zmq_socket (ctx, ZMQ_PUB);
    assert (pub_socket);
    rc = zmq_setsockopt (pub_socket, ZMQ_SNDHWM, &hwm, sizeof (hwm));
    assert (rc == 0);
    rc = zmq_bind (pub_socket, "tcp://127.0.0.1:1234");
    assert (rc == 0);
    
    // Set up connect socket
    void *sub_socket = zmq_socket (ctx, ZMQ_SUB);
    assert (sub_socket);
    rc = zmq_setsockopt (sub_socket, ZMQ_RCVHWM, &hwm, sizeof (hwm));
    assert (rc == 0);
    rc = zmq_connect (sub_socket, "tcp://127.0.0.1:1234");
    assert (rc == 0);
    rc = zmq_setsockopt( sub_socket, ZMQ_SUBSCRIBE, 0, 0);
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    
    // Send messages
    int send_count = 0;
    while (send_count < first_count && zmq_send (pub_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    assert (first_count == send_count);
    
    msleep (SETTLE_TIME);
    
    // Now receive all sent messages
    int recv_count = 0;
    while (0 == zmq_recv (sub_socket, NULL, 0, ZMQ_DONTWAIT))
    {
        ++recv_count;
    }
    assert (first_count == recv_count);
    
    msleep (SETTLE_TIME);
    
    // Send messages
    send_count = 0;
    while (send_count < second_count && zmq_send (pub_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    assert (second_count == send_count);
    
    msleep (SETTLE_TIME);
    
    // Now receive all sent messages
    recv_count = 0;
    while (0 == zmq_recv (sub_socket, NULL, 0, ZMQ_DONTWAIT))
    {
        ++recv_count;
    }
    assert (second_count == recv_count);
    
    // Clean up
    rc = zmq_close (sub_socket);
    assert (rc == 0);
    
    rc = zmq_close (pub_socket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

int test_hwm_pubsub (void)
{
    NSLog(@"- test_hwm_pubsub");
    setup_test_environment();
    
    int count;
    
    // send 1000 msg on hwm 1000, receive 1000
    count = test_defaults (1000,1000);
    assert (count == 1000);
    
    // send 6000 msg on hwm 2000, drops above hwm, only receive hwm
    count = test_blocking (2000,6000);
    assert (count == 6000);
    
    // hwm should apply to the messages that have already been received
    test_reset_hwm ();
    
    return 0;
}

void test_immediate (void)
{
    NSLog(@"- test_immediate");
    setup_test_environment();
    int val;
    int rc;
    char buffer[16];
    // TEST 1.
    // First we're going to attempt to send messages to two
    // pipes, one connected, the other not. We should see
    // the PUSH load balancing to both pipes, and hence half
    // of the messages getting queued, as connect() creates a
    // pipe immediately.
    
    void *context = zmq_ctx_new();
    assert (context);
    void *to = zmq_socket(context, ZMQ_PULL);
    assert (to);
    
    // Bind the one valid receiver
    val = 0;
    rc = zmq_setsockopt(to, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    rc = zmq_bind (to, "tcp://127.0.0.1:6555");
    assert (rc == 0);
    
    // Create a socket pushing to two endpoints - only 1 message should arrive.
    void *from = zmq_socket (context, ZMQ_PUSH);
    assert(from);
    
    val = 0;
    zmq_setsockopt (from, ZMQ_LINGER, &val, sizeof (val));
    // This pipe will not connect
    rc = zmq_connect (from, "tcp://localhost:5556");
    assert (rc == 0);
    // This pipe will
    rc = zmq_connect (from, "tcp://localhost:6555");
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    
    // We send 10 messages, 5 should just get stuck in the queue
    // for the not-yet-connected pipe
    for (int i = 0; i < 10; ++i) {
        rc = zmq_send (from, "Hello", 5, 0);
        assert (rc == 5);
    }
    
    // We now consume from the connected pipe
    // - we should see just 5
    int timeout = 250;
    rc = zmq_setsockopt (to, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    int seen = 0;
    while (true) {
        rc = zmq_recv (to, &buffer, sizeof (buffer), 0);
        if (rc == -1)
            break;          //  Break when we didn't get a message
        seen++;
    }
    assert (seen == 5);
    
    rc = zmq_close (from);
    assert (rc == 0);
    
    rc = zmq_close (to);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    
    // TEST 2
    // This time we will do the same thing, connect two pipes,
    // one of which will succeed in connecting to a bound
    // receiver, the other of which will fail. However, we will
    // also set the delay attach on connect flag, which should
    // cause the pipe attachment to be delayed until the connection
    // succeeds.
    context = zmq_ctx_new();
    
    // Bind the valid socket
    to = zmq_socket (context, ZMQ_PULL);
    assert (to);
    rc = zmq_bind (to, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    val = 0;
    rc = zmq_setsockopt (to, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    
    // Create a socket pushing to two endpoints - all messages should arrive.
    from = zmq_socket (context, ZMQ_PUSH);
    assert (from);
    
    val = 0;
    rc = zmq_setsockopt (from, ZMQ_LINGER, &val, sizeof(val));
    assert (rc == 0);
    
    // Set the key flag
    val = 1;
    rc = zmq_setsockopt (from, ZMQ_IMMEDIATE, &val, sizeof(val));
    assert (rc == 0);
    
    // Connect to the invalid socket
    rc = zmq_connect (from, "tcp://localhost:5561");
    assert (rc == 0);
    // Connect to the valid socket
    rc = zmq_connect (from, "tcp://localhost:5560");
    assert (rc == 0);
    
    // Send 10 messages, all should be routed to the connected pipe
    for (int i = 0; i < 10; ++i) {
        rc = zmq_send (from, "Hello", 5, 0);
        assert (rc == 5);
    }
    rc = zmq_setsockopt (to, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    seen = 0;
    while (true) {
        rc = zmq_recv (to, &buffer, sizeof (buffer), 0);
        if (rc == -1)
            break;          //  Break when we didn't get a message
        seen++;
    }
    assert (seen == 10);
    
    rc = zmq_close (from);
    assert (rc == 0);
    
    rc = zmq_close (to);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    
    // TEST 3
    // This time we want to validate that the same blocking behaviour
    // occurs with an existing connection that is broken. We will send
    // messages to a connected pipe, disconnect and verify the messages
    // block. Then we reconnect and verify messages flow again.
    context = zmq_ctx_new ();
    
    void *backend = zmq_socket (context, ZMQ_DEALER);
    assert (backend);
    void *frontend = zmq_socket (context, ZMQ_DEALER);
    assert (frontend);
    int zero = 0;
    rc = zmq_setsockopt (backend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_setsockopt (frontend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    
    //  Frontend connects to backend using IMMEDIATE
    int on = 1;
    rc = zmq_setsockopt (frontend, ZMQ_IMMEDIATE, &on, sizeof (on));
    assert (rc == 0);
    rc = zmq_bind (backend, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_connect (frontend, "tcp://localhost:5560");
    assert (rc == 0);
    
    //  Ping backend to frontend so we know when the connection is up
    rc = zmq_send (backend, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (frontend, buffer, 255, 0);
    assert (rc == 5);
    
    // Send message from frontend to backend
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == 5);
    
    rc = zmq_close (backend);
    assert (rc == 0);
    
    //  Give time to process disconnect
    msleep (SETTLE_TIME * 10);
    
    // Send a message, should fail
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == -1);
    
    //  Recreate backend socket
    backend = zmq_socket (context, ZMQ_DEALER);
    assert (backend);
    rc = zmq_setsockopt (backend, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_bind (backend, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Ping backend to frontend so we know when the connection is up
    rc = zmq_send (backend, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (frontend, buffer, 255, 0);
    assert (rc == 5);
    
    // After the reconnect, should succeed
    rc = zmq_send (frontend, "Hello", 5, ZMQ_DONTWAIT);
    assert (rc == 5);
    
    rc = zmq_close (backend);
    assert (rc == 0);
    
    rc = zmq_close (frontend);
    assert (rc == 0);
    
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    
}



static void pusher (void *ctx)
{
    // Connect first
    void *connectSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, "inproc://sink");
    assert (rc == 0);
    
    // Queue up some data
    rc = zmq_send_const (connectSocket, "foobar", 6, 0);
    assert (rc == 6);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
}

static void simult_conn (void *payload)
{
    // Pull out arguments - context followed by endpoint string
    void* ctx   = (void*)((void**)payload)[0];
    char* endpt = (char*)((void**)payload)[1];
    
    // Connect
    void *connectSocket = zmq_socket (ctx, ZMQ_SUB);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, endpt);
    assert (rc == 0);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
}

static void simult_bind (void *payload)
{
    // Pull out arguments - context followed by endpoint string
    void* ctx   = (void*)((void**)payload)[0];
    char* endpt = (char*)((void**)payload)[1];
    
    // Bind
    void *bindSocket = zmq_socket (ctx, ZMQ_PUB);
    assert (bindSocket);
    int rc = zmq_bind (bindSocket, endpt);
    assert (rc == 0);
    
    // Cleanup
    rc = zmq_close (bindSocket);
    assert (rc == 0);
}

void test_bind_before_connect ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    // Bind first
    void *bindSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (bindSocket);
    int rc = zmq_bind (bindSocket, "inproc://bbc");
    assert (rc == 0);
    
    // Now connect
    void *connectSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (connectSocket);
    rc = zmq_connect (connectSocket, "inproc://bbc");
    assert (rc == 0);
    
    // Queue up some data
    rc = zmq_send_const (connectSocket, "foobar", 6, 0);
    assert (rc == 6);
    
    // Read pending message
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, bindSocket, 0);
    assert (rc == 6);
    void *data = zmq_msg_data (&msg);
    assert (memcmp ("foobar", data, 6) == 0);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    
    rc = zmq_close (bindSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_connect_before_bind ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    // Connect first
    void *connectSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, "inproc://cbb");
    assert (rc == 0);
    
    // Queue up some data
    rc = zmq_send_const (connectSocket, "foobar", 6, 0);
    assert (rc == 6);
    
    // Now bind
    void *bindSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (bindSocket);
    rc = zmq_bind (bindSocket, "inproc://cbb");
    assert (rc == 0);
    
    // Read pending message
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, bindSocket, 0);
    assert (rc == 6);
    void *data = zmq_msg_data (&msg);
    assert (memcmp ("foobar", data, 6) == 0);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    
    rc = zmq_close (bindSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_connect_before_bind_pub_sub ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    // Connect first
    void *connectSocket = zmq_socket (ctx, ZMQ_PUB);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, "inproc://cbbps");
    assert (rc == 0);
    
    // Queue up some data, this will be dropped
    rc = zmq_send_const (connectSocket, "before", 6, 0);
    assert (rc == 6);
    
    // Now bind
    void *bindSocket = zmq_socket (ctx, ZMQ_SUB);
    assert (bindSocket);
    rc = zmq_setsockopt (bindSocket, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_bind (bindSocket, "inproc://cbbps");
    assert (rc == 0);
    
    // Wait for pub-sub connection to happen
    msleep (SETTLE_TIME);
    
    // Queue up some data, this not will be dropped
    rc = zmq_send_const (connectSocket, "after", 6, 0);
    assert (rc == 6);
    
    // Read pending message
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, bindSocket, 0);
    assert (rc == 6);
    void *data = zmq_msg_data (&msg);
    assert (memcmp ("after", data, 5) == 0);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    
    rc = zmq_close (bindSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_connect_before_bind_ctx_term ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    for (int i = 0; i < 20; ++i) {
        // Connect first
        void *connectSocket = zmq_socket (ctx, ZMQ_ROUTER);
        assert (connectSocket);
        
        char ep[20];
        sprintf(ep, "inproc://cbbrr%d", i);
        int rc = zmq_connect (connectSocket, ep);
        assert (rc == 0);
        
        // Cleanup
        rc = zmq_close (connectSocket);
        assert (rc == 0);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multiple_connects ()
{
    const unsigned int no_of_connects = 10;
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    int rc;
    void *connectSocket [no_of_connects];
    
    // Connect first
    for (unsigned int i = 0; i < no_of_connects; ++i)
    {
        connectSocket [i] = zmq_socket (ctx, ZMQ_PUSH);
        assert (connectSocket [i]);
        rc = zmq_connect (connectSocket [i], "inproc://multiple");
        assert (rc == 0);
        
        // Queue up some data
        rc = zmq_send_const (connectSocket [i], "foobar", 6, 0);
        assert (rc == 6);
    }
    
    // Now bind
    void *bindSocket = zmq_socket (ctx, ZMQ_PULL);
    assert (bindSocket);
    rc = zmq_bind (bindSocket, "inproc://multiple");
    assert (rc == 0);
    
    for (unsigned int i = 0; i < no_of_connects; ++i)
    {
        // Read pending message
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_msg_recv (&msg, bindSocket, 0);
        assert (rc == 6);
        void *data = zmq_msg_data (&msg);
        assert (memcmp ("foobar", data, 6) == 0);
    }
    
    // Cleanup
    for (unsigned int i = 0; i < no_of_connects; ++i)
    {
        rc = zmq_close (connectSocket [i]);
        assert (rc == 0);
    }
    
    rc = zmq_close (bindSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multiple_threads ()
{
    const unsigned int no_of_threads = 30;
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    int rc;
    void *threads [no_of_threads];
    
    // Connect first
    for (unsigned int i = 0; i < no_of_threads; ++i)
    {
        threads [i] = zmq_threadstart (&pusher, ctx);
    }
    
    // Now bind
    void *bindSocket = zmq_socket (ctx, ZMQ_PULL);
    assert (bindSocket);
    rc = zmq_bind (bindSocket, "inproc://sink");
    assert (rc == 0);
    
    for (unsigned int i = 0; i < no_of_threads; ++i)
    {
        // Read pending message
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_msg_recv (&msg, bindSocket, 0);
        assert (rc == 6);
        void *data = zmq_msg_data (&msg);
        assert (memcmp ("foobar", data, 6) == 0);
    }
    
    // Cleanup
    for (unsigned int i = 0; i < no_of_threads; ++i)
    {
        zmq_threadclose (threads [i]);
    }
    
    rc = zmq_close (bindSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_simultaneous_connect_bind_threads ()
{
    const unsigned int no_of_times = 50;
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *threads[no_of_times*2];
    void *thr_args[no_of_times][2];
    char endpts[no_of_times][20];
    
    // Set up thread arguments: context followed by endpoint string
    for (unsigned int i = 0; i < no_of_times; ++i)
    {
        thr_args[i][0] = (void*) ctx;
        thr_args[i][1] = (void*) endpts[i];
        sprintf (endpts[i], "inproc://foo_%d", i);
    }
    
    // Spawn all threads as simultaneously as possible
    for (unsigned int i = 0; i < no_of_times; ++i)
    {
        threads[i*2+0] = zmq_threadstart (&simult_conn, (void*)thr_args[i]);
        threads[i*2+1] = zmq_threadstart (&simult_bind, (void*)thr_args[i]);
    }
    
    // Close all threads
    for (unsigned int i = 0; i < no_of_times; ++i)
    {
        zmq_threadclose (threads[i*2+0]);
        zmq_threadclose (threads[i*2+1]);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_identity ()
{
    //  Create the infrastructure
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    
    int rc = zmq_connect (sc, "inproc://identity");
    assert (rc == 0);
    
    void *sb = zmq_socket (ctx, ZMQ_ROUTER);
    assert (sb);
    
    rc = zmq_bind (sb, "inproc://identity");
    assert (rc == 0);
    
    //  Send 2-part message.
    rc = zmq_send (sc, "A", 1, ZMQ_SNDMORE);
    assert (rc == 1);
    rc = zmq_send (sc, "B", 1, 0);
    assert (rc == 1);
    
    //  Identity comes first.
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc >= 0);
    int more = zmq_msg_more (&msg);
    assert (more == 1);
    
    //  Then the first part of the message body.
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 1);
    more = zmq_msg_more (&msg);
    assert (more == 1);
    
    //  And finally, the second part of the message body.
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 1);
    more = zmq_msg_more (&msg);
    assert (more == 0);
    
    //  Deallocate the infrastructure.
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_connect_only ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *connectSocket = zmq_socket (ctx, ZMQ_PUSH);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, "inproc://a");
    assert (rc == 0);
    
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}


void test_unbind ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    // Bind and unbind socket 1
    void *bindSocket1 = zmq_socket (ctx, ZMQ_PAIR);
    assert (bindSocket1);
    int rc = zmq_bind (bindSocket1, "inproc://unbind");
    assert (rc == 0);
    zmq_unbind (bindSocket1, "inproc://unbind");
    assert (rc == 0);
    
    // Bind socket 2
    void *bindSocket2 = zmq_socket (ctx, ZMQ_PAIR);
    assert (bindSocket2);
    rc = zmq_bind (bindSocket2, "inproc://unbind");
    assert (rc == 0);
    
    // Now connect
    void *connectSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (connectSocket);
    rc = zmq_connect (connectSocket, "inproc://unbind");
    assert (rc == 0);
    
    // Queue up some data
    rc = zmq_send_const (connectSocket, "foobar", 6, 0);
    assert (rc == 6);
    
    // Read pending message
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, bindSocket2, 0);
    assert (rc == 6);
    void *data = zmq_msg_data (&msg);
    assert (memcmp ("foobar", data, 6) == 0);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    rc = zmq_close (bindSocket1);
    assert (rc == 0);
    rc = zmq_close (bindSocket2);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_shutdown_during_pend ()
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    // Connect first
    void *connectSocket = zmq_socket (ctx, ZMQ_PAIR);
    assert (connectSocket);
    int rc = zmq_connect (connectSocket, "inproc://cbb");
    assert (rc == 0);
    
    zmq_ctx_shutdown (ctx);
    
    // Cleanup
    rc = zmq_close (connectSocket);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

int test_inproc_connect (void)
{
    NSLog(@"- test_inproc_connect");
    setup_test_environment ();
    
    test_bind_before_connect ();
    test_connect_before_bind ();
    test_connect_before_bind_pub_sub ();
    test_connect_before_bind_ctx_term ();
    test_multiple_connects ();
    test_multiple_threads ();
    test_simultaneous_connect_bind_threads ();
    test_identity ();
    test_connect_only ();
    test_unbind ();
    test_shutdown_during_pend ();
    
    return 0;
}

int test_invalid_rep(void)
{
    NSLog(@"- test_invalid_rep");
    setup_test_environment();
    //  Create REQ/ROUTER wiring.
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *router_socket = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router_socket);
    
    void *req_socket = zmq_socket (ctx, ZMQ_REQ);
    assert (req_socket);
    
    int linger = 0;
    int rc = zmq_setsockopt (router_socket, ZMQ_LINGER, &linger, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (req_socket, ZMQ_LINGER, &linger, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (router_socket, "inproc://hi");
    assert (rc == 0);
    rc = zmq_connect (req_socket, "inproc://hi");
    assert (rc == 0);
    
    //  Initial request.
    rc = zmq_send (req_socket, "r", 1, 0);
    assert (rc == 1);
    
    //  Receive the request.
    char addr [32];
    int addr_size;
    char bottom [1];
    char body [1];
    addr_size = zmq_recv (router_socket, addr, sizeof (addr), 0);
    assert (addr_size >= 0);
    rc = zmq_recv (router_socket, bottom, sizeof (bottom), 0);
    assert (rc == 0);
    rc = zmq_recv (router_socket, body, sizeof (body), 0);
    assert (rc == 1);
    
    //  Send invalid reply.
    rc = zmq_send (router_socket, addr, addr_size, 0);
    assert (rc == addr_size);
    
    //  Send valid reply.
    rc = zmq_send (router_socket, addr, addr_size, ZMQ_SNDMORE);
    assert (rc == addr_size);
    rc = zmq_send (router_socket, bottom, 0, ZMQ_SNDMORE);
    assert (rc == 0);
    rc = zmq_send (router_socket, "b", 1, 0);
    assert (rc == 1);
    
    //  Check whether we've got the valid reply.
    rc = zmq_recv (req_socket, body, sizeof (body), 0);
    assert (rc == 1);
    assert (body [0] == 'b');
    
    //  Tear down the wiring.
    rc = zmq_close (router_socket);
    assert (rc == 0);
    rc = zmq_close (req_socket);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}



// XSI vector I/O
#if defined ZMQ_HAVE_UIO
#include <sys/uio.h>
#else
struct iovec {
    void *iov_base;
    size_t iov_len;
};
#endif

void do_check(void* sb, void* sc, size_t msg_size)
{
    assert (sb && sc && msg_size > 0);
    
    int rc = 0;
    const char msg_val = '1';
    const int num_messages = 10;
    size_t send_count, recv_count;
    
    send_count = recv_count = num_messages;
    
    char *ref_msg = (char *) malloc (msg_size);
    assert (ref_msg);
    memset (ref_msg, msg_val, msg_size);
    
    // zmq_sendiov(3) as a single multi-part send
    struct iovec send_iov[num_messages];
    char *buf = (char *) malloc (msg_size * num_messages);
    
    for (int i = 0; i < num_messages; i++)
    {
        send_iov[i].iov_base = &buf[i * msg_size];
        send_iov[i].iov_len = msg_size;
        memcpy (send_iov[i].iov_base, ref_msg, msg_size);
        assert (memcmp (ref_msg, send_iov[i].iov_base, msg_size) == 0);
    }
    
    // Test errors - zmq_recviov - null socket
    rc = zmq_sendiov (NULL, send_iov, send_count, ZMQ_SNDMORE);
    assert (rc == -1 && errno == ENOTSOCK);
    // Test errors - zmq_recviov - invalid send count
    rc = zmq_sendiov (sc, send_iov, 0, 0);
    assert (rc == -1 && errno == EINVAL);
    // Test errors - zmq_recviov - null iovec
    rc = zmq_sendiov (sc, NULL, send_count, 0);
    assert (rc == -1 && errno == EINVAL);
    
    // Test success
    rc = zmq_sendiov (sc, send_iov, send_count, ZMQ_SNDMORE);
    // The zmq_sendiov(3) API method does not follow the same semantics as
    // zmq_recviov(3); the latter returns the count of messages sent, rightly
    // so, whilst the former sends the number of bytes successfully sent from
    // the last message, which does not hold much sense from a batch send
    // perspective; hence the assert checks if rc is same as msg_size.
    assert ((size_t)rc == msg_size);
    
    // zmq_recviov(3) single-shot
    struct iovec recv_iov[num_messages];
    
    // Test errors - zmq_recviov - null socket
    rc = zmq_recviov (NULL, recv_iov, &recv_count, 0);
    assert (rc == -1 && errno == ENOTSOCK);
    // Test error - zmq_recviov - invalid receive count
    rc = zmq_recviov (sb, recv_iov, NULL, 0);
    assert (rc == -1 && errno == EINVAL);
    size_t invalid_recv_count = 0;
    rc = zmq_recviov (sb, recv_iov, &invalid_recv_count, 0);
    assert (rc == -1 && errno == EINVAL);
    // Test error - zmq_recviov - null iovec
    rc = zmq_recviov (sb, NULL, &recv_count, 0);
    assert (rc == -1 && errno == EINVAL);
    
    // Test success
    rc = zmq_recviov (sb, recv_iov, &recv_count, 0);
    assert (rc == num_messages);
    
    for (int i = 0; i < num_messages; i++)
    {
        assert (recv_iov[i].iov_base);
        assert (memcmp (ref_msg, recv_iov[i].iov_base, msg_size) == 0);
        free(recv_iov[i].iov_base);
    }
    
    assert (send_count == recv_count);
    free (ref_msg);
    free (buf);
}

int test_iov (void)
{
    NSLog(@"- test_iov");
    setup_test_environment ();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int rc;
    
    void *sb = zmq_socket (ctx, ZMQ_PULL);
    assert (sb);
    
    rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    void *sc = zmq_socket (ctx, ZMQ_PUSH);
    
    rc = zmq_connect (sc, "inproc://a");
    assert (rc == 0);
    
    
    // message bigger than VSM max
    do_check (sb, sc, 100);
    
    // message smaller than VSM max
    do_check (sb, sc, 10);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_ipc_wildcard (void)
{
    NSLog(@"- test_ipc_wildcard");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    int rc = zmq_bind (sb, "ipc://*");
    assert (rc == 0);
    
    char endpoint [200];
    size_t size = sizeof (endpoint);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, endpoint, &size);
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    rc = zmq_connect (sc, endpoint);
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_issue_566(void)
{
    NSLog(@"- test_issue_566");
    setup_test_environment();
    
    void *ctx1 = zmq_ctx_new ();
    assert (ctx1);
    
    void *ctx2 = zmq_ctx_new ();
    assert (ctx2);
    
    void *router = zmq_socket (ctx1, ZMQ_ROUTER);
    int on = 1;
    int rc = zmq_setsockopt (router, ZMQ_ROUTER_MANDATORY, &on, sizeof (on));
    assert (rc == 0);
    rc = zmq_bind (router, "tcp://127.0.0.1:5555");
    assert (rc != -1);
    
    //  Repeat often enough to be sure this works as it should
    for (int cycle = 0; cycle < 100; cycle++) {
        //  Create dealer with unique explicit identity
        //  We assume the router learns this out-of-band
        void *dealer = zmq_socket (ctx2, ZMQ_DEALER);
        char identity [10];
        sprintf (identity, "%09d", cycle);
        rc = zmq_setsockopt (dealer, ZMQ_IDENTITY, identity, 10);
        assert (rc == 0);
        int rcvtimeo = 1000;
        rc = zmq_setsockopt (dealer, ZMQ_RCVTIMEO, &rcvtimeo, sizeof (int));
        assert (rc == 0);
        rc = zmq_connect (dealer, "tcp://127.0.0.1:5555");
        assert (rc == 0);
        
        //  Router will try to send to dealer, at short intervals.
        //  It typically takes 2-5 msec for the connection to establish
        //  on a loopback interface, but we'll allow up to one second
        //  before failing the test (e.g. for running on a debugger or
        //  a very slow system).
        for (int attempt = 0; attempt < 500; attempt++) {
            zmq_poll (0, 0, 2);
            rc = zmq_send (router, identity, 10, ZMQ_SNDMORE);
            if (rc == -1 && errno == EHOSTUNREACH)
                continue;
            assert (rc == 10);
            rc = zmq_send (router, "HELLO", 5, 0);
            assert (rc == 5);
            break;
        }
        uint8_t buffer [5];
        rc = zmq_recv (dealer, buffer, 5, 0);
        assert (rc == 5);
        assert (memcmp (buffer, "HELLO", 5) == 0);
        close_zero_linger (dealer);
    }
    zmq_close (router);
    zmq_ctx_destroy (ctx1);
    zmq_ctx_destroy (ctx2);
    
    return 0;
}


static void do_bind_and_verify (void *s, const char *endpoint)
{
    int rc = zmq_bind (s, endpoint);
    assert (rc == 0);
    char reported [255];
    size_t size = 255;
    rc = zmq_getsockopt (s, ZMQ_LAST_ENDPOINT, reported, &size);
    assert (rc == 0 && strcmp (reported, endpoint) == 0);
}

int test_last_endpoint (void)
{
    NSLog(@"- test_last_endpoint");
    setup_test_environment();
    //  Create the infrastructure
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_ROUTER);
    assert (sb);
    int val = 0;
    int rc = zmq_setsockopt (sb, ZMQ_LINGER, &val, sizeof (val));
    assert (rc == 0);
    
    do_bind_and_verify (sb, "tcp://127.0.0.1:5560");
    do_bind_and_verify (sb, "tcp://127.0.0.1:5561");
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


void test_system_max ()
{
    // Keep allocating sockets until we run out of system resources
    const int no_of_sockets = 2 * 65536;
    void *ctx = zmq_ctx_new ();
    zmq_ctx_set (ctx, ZMQ_MAX_SOCKETS, no_of_sockets);
    std::vector <void*> sockets;
    
    while (true) {
        void *socket = zmq_socket (ctx, ZMQ_PAIR);
        if (!socket)
            break;
        sockets.push_back (socket);
    }
    assert ((int) sockets.size () <= no_of_sockets);
    
    //  System is out of resources, further calls to zmq_socket should return NULL
    for (unsigned int i = 0; i < 10; ++i) {
        void *socket = zmq_socket (ctx, ZMQ_PAIR);
        assert (socket == NULL);
    }
    // Clean up.
    for (unsigned int i = 0; i < sockets.size (); ++i)
        zmq_close (sockets [i]);
    
    zmq_ctx_destroy (ctx);
}

void test_zmq_default_max ()
{
    //  Keep allocating sockets until we hit the default limit
    void *ctx = zmq_ctx_new ();
    std::vector<void*> sockets;
    
    while (true) {
        void *socket = zmq_socket (ctx, ZMQ_PAIR);
        if (!socket)
            break;
        sockets.push_back (socket);
    }
    //  We may stop sooner if system has fewer available sockets
    assert (sockets.size () <= ZMQ_MAX_SOCKETS_DFLT);
    
    //  Further calls to zmq_socket should return NULL
    for (unsigned int i = 0; i < 10; ++i) {
        void *socket = zmq_socket (ctx, ZMQ_PAIR);
        assert (socket == NULL);
    }
    
    //  Clean up
    for (unsigned int i = 0; i < sockets.size (); ++i)
        zmq_close (sockets [i]);
    
    zmq_ctx_destroy (ctx);
}

int test_many_sockets (void)
{
    NSLog(@"- test_many_sockets");
    setup_test_environment ();
    
    test_system_max ();
    test_zmq_default_max ();
    
    return 0;
}


static void
zap_handler (void *handler)
{
    uint8_t metadata [] = {
        5, 'H', 'e', 'l', 'l', 'o',
        0, 0, 0, 5, 'W', 'o', 'r', 'l', 'd'
    };
    
    //  Process ZAP requests forever
    while (true) {
        char *version = s_recv (handler);
        if (!version)
            break;          //  Terminating
        
        char *sequence = s_recv (handler);
        char *domain = s_recv (handler);
        char *address = s_recv (handler);
        char *identity = s_recv (handler);
        char *mechanism = s_recv (handler);
        
        assert (streq (version, "1.0"));
        assert (streq (mechanism, "NULL"));
        
        s_sendmore (handler, version);
        s_sendmore (handler, sequence);
        if (streq (domain, "DOMAIN")) {
            s_sendmore (handler, "200");
            s_sendmore (handler, "OK");
            s_sendmore (handler, "anonymous");
            zmq_send (handler, metadata, sizeof (metadata), 0);
        }
        else {
            s_sendmore (handler, "400");
            s_sendmore (handler, "BAD DOMAIN");
            s_sendmore (handler, "");
            s_send     (handler, "");
        }
        free (version);
        free (sequence);
        free (domain);
        free (address);
        free (identity);
        free (mechanism);
    }
    close_zero_linger (handler);
}

int test_metadata (void)
{
    NSLog(@"- test_medata");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Spawn ZAP handler
    //  We create and bind ZAP socket in main thread to avoid case
    //  where child thread does not start up fast enough.
    void *handler = zmq_socket (ctx, ZMQ_REP);
    assert (handler);
    int rc = zmq_bind (handler, "inproc://zeromq.zap.01");
    assert (rc == 0);
    void *zap_thread = zmq_threadstart (&zap_handler, handler);
    
    void *server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (server, ZMQ_ZAP_DOMAIN, "DOMAIN", 6);
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9001");
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:9001");
    assert (rc == 0);
    
    s_send (client, "This is a message");
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    rc = zmq_msg_recv (&msg, server, 0);
    assert (rc != -1);
    assert (streq (zmq_msg_gets (&msg, "Hello"), "World"));
    assert (streq (zmq_msg_gets (&msg, "Socket-Type"), "DEALER"));
    assert (streq (zmq_msg_gets (&msg, "User-Id"), "anonymous"));
    assert (streq (zmq_msg_gets (&msg, "Peer-Address"), "127.0.0.1"));
    
    assert (zmq_msg_gets (&msg, "No Such") == NULL);
    assert (zmq_errno () == EINVAL);
    zmq_msg_close (&msg);
    
    close_zero_linger (client);
    close_zero_linger (server);
    
    //  Shutdown
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Wait until ZAP handler terminates
    zmq_threadclose (zap_thread);
    
    return 0;
}

//  Read one event off the monitor socket; return value and address
//  by reference, if not null, and event number by value. Returns -1
//  in case of error.

static int
get_monitor_event (void *monitor, int *value, char **address)
{
    //  First frame in message contains event number and value
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (zmq_msg_more (&msg));
    
    uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
    uint16_t event = *(uint16_t *) (data);
    if (value)
        *value = *(uint32_t *) (data + 2);
    
    //  Second frame in message contains event address
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (!zmq_msg_more (&msg));
    
    if (address) {
        uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
        size_t size = zmq_msg_size (&msg);
        *address = (char *) malloc (size + 1);
        memcpy (*address, data, size);
        *address [size] = 0;
    }
    return event;
}

int test_monitor (void)
{
    NSLog(@"- test_monitor");
    setup_test_environment();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  We'll monitor these two sockets
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    void *server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    
    //  Socket monitoring only works over inproc://
    int rc = zmq_socket_monitor (client, "tcp://127.0.0.1:9999", 0);
    assert (rc == -1);
    assert (zmq_errno () == EPROTONOSUPPORT);
    
    //  Monitor all events on client and server sockets
    rc = zmq_socket_monitor (client, "inproc://monitor-client", ZMQ_EVENT_ALL);
    assert (rc == 0);
    rc = zmq_socket_monitor (server, "inproc://monitor-server", ZMQ_EVENT_ALL);
    assert (rc == 0);
    
    //  Create two sockets for collecting monitor events
    void *client_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (client_mon);
    void *server_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (server_mon);
    
    //  Connect these to the inproc endpoints so they'll get events
    rc = zmq_connect (client_mon, "inproc://monitor-client");
    assert (rc == 0);
    rc = zmq_connect (server_mon, "inproc://monitor-server");
    assert (rc == 0);
    
    //  Now do a basic ping test
    rc = zmq_bind (server, "tcp://127.0.0.1:9998");
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:9998");
    assert (rc == 0);
    bounce (server, client);
    
    //  Close client and server
    close_zero_linger (client);
    close_zero_linger (server);
    
    //  Now collect and check events from both sockets
    int event = get_monitor_event (client_mon, NULL, NULL);
    if (event == ZMQ_EVENT_CONNECT_DELAYED)
        event = get_monitor_event (client_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_CONNECTED);
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event (client_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_SUCCEED);
#endif
    event = get_monitor_event (client_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_MONITOR_STOPPED);
    
    //  This is the flow of server events
    event = get_monitor_event (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_LISTENING);
    event = get_monitor_event (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_ACCEPTED);
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_SUCCEED);
#endif
    event = get_monitor_event (server_mon, NULL, NULL);
    //  Sometimes the server sees the client closing before it gets closed.
    if (event != ZMQ_EVENT_DISCONNECTED) {
        assert (event == ZMQ_EVENT_CLOSED);
        event = get_monitor_event (server_mon, NULL, NULL);
    }
    if (event != ZMQ_EVENT_DISCONNECTED) {
        assert (event == ZMQ_EVENT_MONITOR_STOPPED);
    }
    
    //  Close down the sockets
    close_zero_linger (client_mon);
    close_zero_linger (server_mon);
    zmq_ctx_term (ctx);
    
    return 0 ;
}


void ffn(void *data, void *hint) {
    // Signal that ffn has been called by writing "freed" to hint
    (void) data;      //  Suppress 'unused' warnings at compile time
    memcpy(hint, (void *) "freed", 5);
}

int test_msg_ffn (void)
{
    NSLog(@"- test_msg_ffn");
    setup_test_environment();
    //  Create the infrastructure
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    int rc = zmq_bind (router, "tcp://127.0.0.1:5555");
    assert (rc == 0);
    
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    
    rc = zmq_connect (dealer, "tcp://127.0.0.1:5555");
    assert (rc == 0);
    
    // Test that creating and closing a message triggers ffn
    zmq_msg_t msg;
    char hint[5];
    char data[255];
    memset(data, 0, 255);
    memcpy(data, (void *) "data", 4);
    memcpy(hint, (void *) "hint", 4);
    rc = zmq_msg_init_data(&msg, (void *)data, 255, ffn, (void*)hint);
    assert (rc == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    assert (memcmp(hint, "freed", 5) == 0);
    memcpy(hint, (void *) "hint", 4);
    
    // Making and closing a copy triggers ffn
    zmq_msg_t msg2;
    zmq_msg_init(&msg2);
    rc = zmq_msg_init_data(&msg, (void *)data, 255, ffn, (void *)hint);
    assert (rc == 0);
    rc = zmq_msg_copy(&msg2, &msg);
    assert (rc == 0);
    rc = zmq_msg_close(&msg2);
    assert (rc == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    assert (memcmp(hint, "freed", 5) == 0);
    memcpy(hint, (void *) "hint", 4);
    
    // Test that sending a message triggers ffn
    rc = zmq_msg_init_data(&msg, (void *)data, 255, ffn, (void *)hint);
    assert (rc == 0);
    
    zmq_msg_send(&msg, dealer, 0);
    char buf[255];
    rc = zmq_recv(router, buf, 255, 0);
    assert (rc > -1);
    rc = zmq_recv(router, buf, 255, 0);
    assert (rc == 255);
    assert (memcmp(data, buf, 4) == 0);
    
    msleep (SETTLE_TIME);
    assert (memcmp(hint, "freed", 5) == 0);
    memcpy(hint, (void *) "hint", 4);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    // Sending a copy of a message triggers ffn
    rc = zmq_msg_init(&msg2);
    assert (rc == 0);
    rc = zmq_msg_init_data(&msg, (void *)data, 255, ffn, (void *)hint);
    assert (rc == 0);
    rc = zmq_msg_copy(&msg2, &msg);
    assert (rc == 0);
    
    zmq_msg_send(&msg, dealer, 0);
    rc = zmq_recv(router, buf, 255, 0);
    assert (rc > -1);
    rc = zmq_recv(router, buf, 255, 0);
    assert (rc == 255);
    assert (memcmp(data, buf, 4) == 0);
    rc = zmq_msg_close(&msg2);
    assert (rc == 0);
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    assert (memcmp(hint, "freed", 5) == 0);
    memcpy(hint, (void *) "hint", 4);
    
    //  Deallocate the infrastructure.
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    return 0 ;
}

int test_msg_flags (void)
{
    NSLog(@"- test_msg_flags");
    setup_test_environment();
    //  Create the infrastructure
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_ROUTER);
    assert (sb);
    
    int rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    
    rc = zmq_connect (sc, "inproc://a");
    assert (rc == 0);
    
    //  Send 2-part message.
    rc = zmq_send (sc, "A", 1, ZMQ_SNDMORE);
    assert (rc == 1);
    rc = zmq_send (sc, "B", 1, 0);
    assert (rc == 1);
    
    //  Identity comes first.
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc >= 0);
    int more = zmq_msg_more (&msg);
    assert (more == 1);
    
    //  Then the first part of the message body.
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 1);
    more = zmq_msg_more (&msg);
    assert (more == 1);
    
    //  And finally, the second part of the message body.
    rc = zmq_msg_recv (&msg, sb, 0);
    assert (rc == 1);
    more = zmq_msg_more (&msg);
    assert (more == 0);
    
    // Test ZMQ_SHARED property (case 1, refcounted messages)
    zmq_msg_t msg_a;
    rc = zmq_msg_init_size(&msg_a, 1024); // large enough to be a type_lmsg
    assert (rc == 0);
    
    // Message is not shared
    rc = zmq_msg_get(&msg_a, ZMQ_SHARED);
    assert (rc == 0);
    
    zmq_msg_t msg_b;
    rc = zmq_msg_init(&msg_b);
    assert (rc == 0);
    
    rc = zmq_msg_copy(&msg_b, &msg_a);
    assert (rc == 0);
    
    // Message is now shared
    rc = zmq_msg_get(&msg_b, ZMQ_SHARED);
    assert (rc == 1);
    
    // cleanup
    rc = zmq_msg_close(&msg_a);
    assert (rc == 0);
    rc = zmq_msg_close(&msg_b);
    assert (rc == 0);
    
    // Test ZMQ_SHARED property (case 2, constant data messages)
    rc = zmq_msg_init_data(&msg_a, (void*) "TEST", 5, 0, 0);
    assert (rc == 0);
    
    // Message reports as shared
    rc = zmq_msg_get(&msg_a, ZMQ_SHARED);
    assert (rc == 1);
    
    // cleanup
    rc = zmq_msg_close(&msg_a);
    assert (rc == 0);
    
    //  Deallocate the infrastructure.
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    return 0 ;
}

int test_pair_inproc (void)
{
    NSLog(@"- test_pair_inproc");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    int rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    rc = zmq_connect (sc, "inproc://a");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    // Test zmq_send_const
    rc = zmq_send_const (sb, "foo", 3, ZMQ_SNDMORE);
    assert (rc == 3);
    rc = zmq_send_const (sb, "foobar", 6, 0);
    assert (rc == 6);
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    rc = zmq_msg_recv (&msg, sc, 0);
    assert (rc == 3);
    assert (zmq_msg_size (&msg) == 3);
    void* data = zmq_msg_data (&msg);
    assert (memcmp ("foo", data, 3) == 0);
    rc = zmq_msg_recv (&msg, sc, 0);
    assert (rc == 6);
    data = zmq_msg_data (&msg);
    assert (memcmp ("foobar", data, 6) == 0);
    
    // Cleanup
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_pair_ipc (void)
{
    NSLog(@"- test_pair_ipc");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    int rc = zmq_bind (sb, "ipc:///tmp/tester");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    rc = zmq_connect (sc, "ipc:///tmp/tester");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_pair_tcp (void)
{
    NSLog(@"- test_pair_tcp");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    int rc = zmq_bind (sb, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_pair_tipc (void)
{
    NSLog(@"- test_pair_tipc");
    fprintf (stderr, "test_pair_tipc running...\n");
    
    void *ctx = zmq_init (1);
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_PAIR);
    assert (sb);
    int rc = zmq_bind (sb, "tipc://{5560,0,0}");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_PAIR);
    assert (sc);
    rc = zmq_connect (sc, "tipc://{5560,0}@0.0.0");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_poller (void)
{
    NSLog(@"- test_poller");
    setup_test_environment ();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create few sockets
    void *vent = zmq_socket (ctx, ZMQ_PUSH);
    assert (vent);
    int rc = zmq_bind (vent, "tcp://127.0.0.1:55556");
    assert (rc == 0);
    
    void *sink = zmq_socket (ctx, ZMQ_PULL);
    assert (sink);
    rc = zmq_connect (sink, "tcp://127.0.0.1:55556");
    assert (rc == 0);
    
    void *bowl = zmq_socket (ctx, ZMQ_PULL);
    assert (bowl);
    
#if defined(ZMQ_SERVER) && defined(ZMQ_CLIENT)
    void *server = zmq_socket (ctx, ZMQ_SERVER);
    assert (server);
    rc = zmq_bind (server, "tcp://127.0.0.1:55557");
    assert (rc == 0);
    
    void *client = zmq_socket (ctx, ZMQ_CLIENT);
    assert (client);
#endif
    
    //  Set up poller
    void* poller = zmq_poller_new ();
    zmq_poller_event_t event;
    
    // waiting on poller with no registered sockets should report error
    rc = zmq_poller_wait(poller, &event, 0);
    assert (rc == -1);
    assert (errno == ETIMEDOUT);
    
    // register sink
    rc = zmq_poller_add (poller, sink, sink, ZMQ_POLLIN);
    assert (rc == 0);
    
    //  Send a message
    char data[1] = {'H'};
    rc = zmq_send_const (vent, data, 1, 0);
    assert (rc == 1);
    
    //  We expect a message only on the sink
    rc = zmq_poller_wait (poller, &event, -1);
    assert (rc == 0);
    assert (event.socket == sink);
    assert (event.user_data == sink);
    rc = zmq_recv (sink, data, 1, 0);
    assert (rc == 1);
    
    //  We expect timed out
    rc = zmq_poller_wait (poller, &event, 0);
    assert (rc == -1);
    assert (errno == ETIMEDOUT);
    
    //  Stop polling sink
    rc = zmq_poller_remove (poller, sink);
    assert (rc == 0);
    
    //  Check we can poll an FD
    rc = zmq_connect (bowl, "tcp://127.0.0.1:55556");
    assert (rc == 0);
    
#if defined _WIN32
    SOCKET fd;
    size_t fd_size = sizeof (SOCKET);
#else
    int fd;
    size_t fd_size = sizeof (int);
#endif
    
    rc = zmq_getsockopt (bowl, ZMQ_FD, &fd, &fd_size);
    assert (rc == 0);
    rc = zmq_poller_add_fd (poller, fd, bowl, ZMQ_POLLIN);
    assert (rc == 0);
    rc = zmq_poller_wait (poller, &event, 500);
    assert (rc == 0);
    assert (event.socket == NULL);
    assert (event.fd == fd);
    assert (event.user_data == bowl);
    zmq_poller_remove_fd (poller, fd);
    
#if defined(ZMQ_SERVER) && defined(ZMQ_CLIENT)
    //  Polling on thread safe sockets
    rc = zmq_poller_add (poller, server, NULL, ZMQ_POLLIN);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:55557");
    assert (rc == 0);
    rc = zmq_send_const (client, data, 1, 0);
    assert (rc == 1);
    rc = zmq_poller_wait (poller, &event, 500);
    assert (rc == 0);
    assert (event.socket == server);
    assert (event.user_data == NULL);
    rc = zmq_recv (server, data, 1, 0);
    assert (rc == 1);
    
    //  Polling on pollout
    rc = zmq_poller_modify (poller, server, ZMQ_POLLOUT | ZMQ_POLLIN);
    assert (rc == 0);
    rc = zmq_poller_wait (poller, &event, 0);
    assert (rc == 0);
    assert (event.socket == server);
    assert (event.user_data == NULL);
    assert (event.events == ZMQ_POLLOUT);
#endif
    
    //  Destory sockets, poller and ctx
    rc = zmq_close (sink);
    assert (rc == 0);
    rc = zmq_close (vent);
    assert (rc == 0);
    rc = zmq_close (bowl);
    assert (rc == 0);
#if defined(ZMQ_SERVER) && defined(ZMQ_CLIENT)
    rc = zmq_close (server);
    assert (rc == 0);
    rc = zmq_close (client);
    assert (rc == 0);
#endif
    
    // Test error - null poller pointers
    rc = zmq_poller_destroy (NULL);
    assert (rc == -1 && errno == EFAULT);
    void *null_poller = NULL;
    rc = zmq_poller_destroy (&null_poller);
    assert (rc == -1 && errno == EFAULT);
    
    rc = zmq_poller_destroy (&poller);
    assert(rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_probe_route (void)
{
    NSLog(@"- test_probe_route");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create server and bind to endpoint
    void *server = zmq_socket (ctx, ZMQ_ROUTER);
    assert (server);
    int rc = zmq_bind (server, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Create client and connect to server, doing a probe
    void *client = zmq_socket (ctx, ZMQ_ROUTER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_IDENTITY, "X", 1);
    assert (rc == 0);
    int probe = 1;
    rc = zmq_setsockopt (client, ZMQ_PROBE_ROUTER, &probe, sizeof (probe));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:5560");
    assert (rc == 0);
    
    //  We expect an identity=X + empty message from client
    unsigned char buffer [255];
    rc = zmq_recv (server, buffer, 255, 0);
    assert (rc == 1);
    assert (buffer [0] ==  'X');
    rc = zmq_recv (server, buffer, 255, 0);
    assert (rc == 0);
    
    //  Send a message to client now
    rc = zmq_send (server, "X", 1, ZMQ_SNDMORE);
    assert (rc == 1);
    rc = zmq_send (server, "Hello", 5, 0);
    assert (rc == 5);
    
    rc = zmq_recv (client, buffer, 255, 0);
    assert (rc == 5);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


#define CONTENT_SIZE 13
#define CONTENT_SIZE_MAX 32
#define ID_SIZE 10
#define ID_SIZE_MAX 32
#define QT_WORKERS    5
#define QT_CLIENTS    3
#define is_verbose 0

static void
client_task (void *ctx)
{
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_SUB);
    assert (control);
    int rc = zmq_setsockopt (control, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    int linger = 0;
    rc = zmq_setsockopt (control, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_connect (control, "inproc://control");
    assert (rc == 0);
    
    char content [CONTENT_SIZE_MAX];
    // Set random identity to make tracing easier
    char identity [ID_SIZE];
    sprintf (identity, "%04X-%04X", rand() % 0xFFFF, rand() % 0xFFFF);
    rc = zmq_setsockopt (client, ZMQ_IDENTITY, identity, ID_SIZE); // includes '\0' as an helper for printf
    assert (rc == 0);
    linger = 0;
    rc = zmq_setsockopt (client, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:5563");
    assert (rc == 0);
    
    zmq_pollitem_t items [] = { { client, 0, ZMQ_POLLIN, 0 }, { control, 0, ZMQ_POLLIN, 0 } };
    int request_nbr = 0;
    bool run = true;
    while (run) {
        // Tick once per 200 ms, pulling in arriving messages
        int centitick;
        for (centitick = 0; centitick < 20; centitick++) {
            zmq_poll (items, 2, 10);
            if (items [0].revents & ZMQ_POLLIN) {
                int rcvmore;
                size_t sz = sizeof (rcvmore);
                rc = zmq_recv (client, content, CONTENT_SIZE_MAX, 0);
                assert (rc == CONTENT_SIZE);
                if (is_verbose) printf("client receive - identity = %s    content = %s\n", identity, content);
                //  Check that message is still the same
                assert (memcmp (content, "request #", 9) == 0);
                rc = zmq_getsockopt (client, ZMQ_RCVMORE, &rcvmore, &sz);
                assert (rc == 0);
                assert (!rcvmore);
            }
            if (items [1].revents & ZMQ_POLLIN) {
                rc = zmq_recv (control, content, CONTENT_SIZE_MAX, 0);
                if (is_verbose) printf("client receive - identity = %s    command = %s\n", identity, content);
                if (memcmp (content, "TERMINATE", 9) == 0) {
                    run = false;
                    break;
                }
            }
        }
        sprintf(content, "request #%03d", ++request_nbr); // CONTENT_SIZE
        rc = zmq_send (client, content, CONTENT_SIZE, 0);
        assert (rc == CONTENT_SIZE);
    }
    
    rc = zmq_close (client);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
}

// This is our server task.
// It uses the multithreaded server model to deal requests out to a pool
// of workers and route replies back to clients. One worker can handle
// one request at a time but one client can talk to multiple workers at
// once.

static void server_worker (void *ctx);

void
server_task (void *ctx)
{
    // Frontend socket talks to clients over TCP
    void *frontend = zmq_socket (ctx, ZMQ_ROUTER);
    assert (frontend);
    int linger = 0;
    int rc = zmq_setsockopt (frontend, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_bind (frontend, "tcp://127.0.0.1:5563");
    assert (rc == 0);
    
    // Backend socket talks to workers over inproc
    void *backend = zmq_socket (ctx, ZMQ_DEALER);
    assert (backend);
    rc = zmq_setsockopt (backend, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_bind (backend, "inproc://backend");
    assert (rc == 0);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_SUB);
    assert (control);
    rc = zmq_setsockopt (control, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_setsockopt (control, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_connect (control, "inproc://control");
    assert (rc == 0);
    
    // Launch pool of worker threads, precise number is not critical
    int thread_nbr;
    void* threads [5];
    for (thread_nbr = 0; thread_nbr < QT_WORKERS; thread_nbr++)
        threads[thread_nbr] = zmq_threadstart (&server_worker, ctx);
    
    // Connect backend to frontend via a proxy
    rc = zmq_proxy_steerable (frontend, backend, NULL, control);
    assert (rc == 0);
    
    for (thread_nbr = 0; thread_nbr < QT_WORKERS; thread_nbr++)
        zmq_threadclose (threads[thread_nbr]);
    
    rc = zmq_close (frontend);
    assert (rc == 0);
    rc = zmq_close (backend);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
}

// Each worker task works on one request at a time and sends a random number
// of replies back, with random delays between replies:
// The comments in the first column, if suppressed, makes it a poller version

static void
server_worker (void *ctx)
{
    void *worker = zmq_socket (ctx, ZMQ_DEALER);
    assert (worker);
    int linger = 0;
    int rc = zmq_setsockopt (worker, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_connect (worker, "inproc://backend");
    assert (rc == 0);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_SUB);
    assert (control);
    rc = zmq_setsockopt (control, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_setsockopt (control, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_connect (control, "inproc://control");
    assert (rc == 0);
    
    char content [CONTENT_SIZE_MAX]; //    bigger than what we need to check that
    char identity [ID_SIZE_MAX];      // the size received is the size sent
    
    bool run = true;
    while (run) {
        rc = zmq_recv (control, content, CONTENT_SIZE_MAX, ZMQ_DONTWAIT); // usually, rc == -1 (no message)
        if (rc > 0) {
            if (is_verbose)
                printf("server_worker receives command = %s\n", content);
            if (memcmp (content, "TERMINATE", 9) == 0)
                run = false;
        }
        // The DEALER socket gives us the reply envelope and message
        // if we don't poll, we have to use ZMQ_DONTWAIT, if we poll, we can block-receive with 0
        rc = zmq_recv (worker, identity, ID_SIZE_MAX, ZMQ_DONTWAIT);
        if (rc == ID_SIZE) {
            rc = zmq_recv (worker, content, CONTENT_SIZE_MAX, 0);
            assert (rc == CONTENT_SIZE);
            if (is_verbose)
                printf ("server receive - identity = %s    content = %s\n", identity, content);
            
            // Send 0..4 replies back
            int reply, replies = rand() % 5;
            for (reply = 0; reply < replies; reply++) {
                // Sleep for some fraction of a second
                msleep (rand () % 10 + 1);
                //  Send message from server to client
                rc = zmq_send (worker, identity, ID_SIZE, ZMQ_SNDMORE);
                assert (rc == ID_SIZE);
                rc = zmq_send (worker, content, CONTENT_SIZE, 0);
                assert (rc == CONTENT_SIZE);
            }
        }
    }
    rc = zmq_close (worker);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
}

// The main thread simply starts several clients and a server, and then
// waits for the server to finish.

int test_proxy (void)
{
    NSLog(@"- test_proxy");
    setup_test_environment ();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_PUB);
    assert (control);
    int linger = 0;
    int rc = zmq_setsockopt (control, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    rc = zmq_bind (control, "inproc://control");
    assert (rc == 0);
    
    void *threads [QT_CLIENTS + 1];
    for (int i = 0; i < QT_CLIENTS; i++)
        threads[i] = zmq_threadstart  (&client_task, ctx);
    threads[QT_CLIENTS] = zmq_threadstart  (&server_task, ctx);
    msleep (500); // Run for 500 ms then quit
    
    rc = zmq_send (control, "TERMINATE", 9, 0);
    assert (rc == 9);
    
    rc = zmq_close (control);
    assert (rc == 0);
    
    for (int i = 0; i < QT_CLIENTS + 1; i++)
        zmq_threadclose (threads[i]);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    return 0;
}

// This is our server task.
// It runs a proxy with a single REP socket as both frontend and backend.

void
server_task1 (void *ctx)
{
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    int rc = zmq_bind (rep, "tcp://127.0.0.1:5563");
    assert (rc == 0);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_SUB);
    assert (control);
    rc = zmq_setsockopt (control, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_connect (control, "inproc://control");
    assert (rc == 0);
    
    // Use rep as both frontend and backend
    rc = zmq_proxy_steerable (rep, rep, NULL, control);
    assert (rc == 0);
    
    rc = zmq_close (rep);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
}


// The main thread simply starts several clients and a server, and then
// waits for the server to finish.

int test_proxy_single_socket (void)
{
    NSLog(@"- test_proxy_single_socket");
    setup_test_environment ();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    // client socket pings proxy over tcp
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    int rc = zmq_connect (req, "tcp://127.0.0.1:5563");
    assert (rc == 0);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_PUB);
    assert (control);
    rc = zmq_bind (control, "inproc://control");
    assert (rc == 0);
    
    void *server_thread = zmq_threadstart(&server_task1, ctx);
    
    char buf[255];
    rc = zmq_send(req, "msg1", 4, 0);
    assert (rc == 4);
    rc = zmq_recv(req, buf, 255, 0);
    assert (rc == 4);
    assert (memcmp (buf, "msg1", 4) == 0);
    
    rc = zmq_send(req, "msg22", 5, 0);
    assert (rc == 5);
    rc = zmq_recv(req, buf, 255, 0);
    assert (rc == 5);
    assert (memcmp (buf, "msg22", 5) == 0);
    
    rc = zmq_send (control, "TERMINATE", 9, 0);
    assert (rc == 9);
    
    rc = zmq_close (control);
    assert (rc == 0);
    rc = zmq_close (req);
    assert (rc == 0);
    
    zmq_threadclose (server_thread);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    return 0;
}


// This is a test for issue #1382. The server thread creates a SUB-PUSH
// steerable proxy. The main process then sends messages to the SUB
// but there is no pull on the other side, previously the proxy blocks
// in writing to the backend, preventing the proxy from terminating

void
server_task2 (void *ctx)
{
    // Frontend socket talks to main process
    void *frontend = zmq_socket (ctx, ZMQ_SUB);
    assert (frontend);
    int rc = zmq_setsockopt (frontend, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_bind (frontend, "tcp://127.0.0.1:15564");
    assert (rc == 0);
    
    // Nice socket which is never read
    void *backend = zmq_socket (ctx, ZMQ_PUSH);
    assert (backend);
    rc = zmq_bind (backend, "tcp://127.0.0.1:15563");
    assert (rc == 0);
    
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_SUB);
    assert (control);
    rc = zmq_setsockopt (control, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    rc = zmq_connect (control, "inproc://control");
    assert (rc == 0);
    
    // Connect backend to frontend via a proxy
    rc = zmq_proxy_steerable (frontend, backend, NULL, control);
    assert (rc == 0);
    
    rc = zmq_close (frontend);
    assert (rc == 0);
    rc = zmq_close (backend);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
}


// The main thread simply starts a basic steerable proxy server, publishes some messages, and then
// waits for the server to terminate.

int test_proxy_terminate (void)
{
    NSLog(@"- test_proxy_terminate");

    setup_test_environment ();
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    // Control socket receives terminate command from main over inproc
    void *control = zmq_socket (ctx, ZMQ_PUB);
    assert (control);
    int rc = zmq_bind (control, "inproc://control");
    assert (rc == 0);
    
    void *thread = zmq_threadstart(&server_task2, ctx);
    msleep (500); // Run for 500 ms
    
    // Start a secondary publisher which writes data to the SUB-PUSH server socket
    void *publisher = zmq_socket (ctx, ZMQ_PUB);
    assert (publisher);
    rc = zmq_connect (publisher, "tcp://127.0.0.1:15564");
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    rc = zmq_send (publisher, "This is a test", 14, 0);
    assert (rc == 14);
    
    msleep (50);
    rc = zmq_send (publisher, "This is a test", 14, 0);
    assert (rc == 14);
    
    msleep (50);
    rc = zmq_send (publisher, "This is a test", 14, 0);
    assert (rc == 14);
    rc = zmq_send (control, "TERMINATE", 9, 0);
    assert (rc == 9);
    
    rc = zmq_close (publisher);
    assert (rc == 0);
    rc = zmq_close (control);
    assert (rc == 0);
    
    zmq_threadclose (thread);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    return 0;
}

int test_pub_invert_matching (void)
{
    NSLog(@"- test_pub_invert_matching");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create a publisher
    void *pub = zmq_socket (ctx, ZMQ_PUB);
    assert (pub);
    int rc = zmq_bind (pub, "inproc://soname");
    assert (rc == 0);
    
    //  Create two subscribers
    void *sub1 = zmq_socket (ctx, ZMQ_SUB);
    assert (sub1);
    rc = zmq_connect (sub1, "inproc://soname");
    assert (rc == 0);
    
    void *sub2 = zmq_socket (ctx, ZMQ_SUB);
    assert (sub2);
    rc = zmq_connect (sub2, "inproc://soname");
    assert (rc == 0);
    
    //  Subscribe pub1 to one prefix
    //  and pub2 to another prefix.
    const char PREFIX1[] = "prefix1";
    const char PREFIX2[] = "p2";
    
    rc = zmq_setsockopt (sub1, ZMQ_SUBSCRIBE, PREFIX1, sizeof(PREFIX1));
    assert (rc == 0);
    
    rc = zmq_setsockopt (sub2, ZMQ_SUBSCRIBE, PREFIX2, sizeof(PREFIX2));
    assert (rc == 0);
    
    //  Send a message with the first prefix
    rc = zmq_send_const(pub, PREFIX1, sizeof(PREFIX1), 0);
    assert (rc == sizeof(PREFIX1));
    
    //  sub1 should receive it, but not sub2
    rc = zmq_recv (sub1, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == sizeof(PREFIX1));
    
    rc = zmq_recv (sub2, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    //  Send a message with the second prefix
    rc = zmq_send_const(pub, PREFIX2, sizeof(PREFIX2), 0);
    assert (rc == sizeof(PREFIX2));
    
    //  sub2 should receive it, but not sub1
    rc = zmq_recv (sub2, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == sizeof(PREFIX2));
    
    rc = zmq_recv (sub1, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    //  Now invert the matching
    int invert = 1;
    rc = zmq_setsockopt (pub, ZMQ_INVERT_MATCHING, &invert, sizeof(invert));
    assert (rc == 0);
    
    //  ... on both sides, otherwise the SUB socket will filter the messages out
    rc = zmq_setsockopt (sub1, ZMQ_INVERT_MATCHING, &invert, sizeof(invert));
    rc = zmq_setsockopt (sub2, ZMQ_INVERT_MATCHING, &invert, sizeof(invert));
    assert (rc == 0);
    
    //  Send a message with the first prefix
    rc = zmq_send_const(pub, PREFIX1, sizeof(PREFIX1), 0);
    assert (rc == sizeof(PREFIX1));
    
    //  sub2 should receive it, but not sub1
    rc = zmq_recv (sub2, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == sizeof(PREFIX1));
    
    rc = zmq_recv (sub1, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    //  Send a message with the second prefix
    rc = zmq_send_const(pub, PREFIX2, sizeof(PREFIX2), 0);
    assert (rc == sizeof(PREFIX2));
    
    //  sub1 should receive it, but not sub2
    rc = zmq_recv (sub1, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == sizeof(PREFIX2));
    
    rc = zmq_recv (sub2, NULL, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    
    //  Clean up.
    rc = zmq_close (pub);
    assert (rc == 0);
    rc = zmq_close (sub1);
    assert (rc == 0);
    rc = zmq_close (sub2);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int msg_send2 (zmq_msg_t *msg_, void *s_, const char* group_, const char* body_)
{
    int rc = zmq_msg_init_size (msg_, strlen (body_));
    if (rc != 0)
        return rc;
    
    memcpy (zmq_msg_data (msg_), body_, strlen (body_));
    
    rc = zmq_msg_set_group (msg_, group_);
    if (rc != 0) {
        zmq_msg_close (msg_);
        return rc;
    }
    
    rc = zmq_msg_send (msg_, s_, 0);
    
    zmq_msg_close (msg_);
    
    return rc;
}

int msg_recv_cmp2 (zmq_msg_t *msg_, void *s_, const char* group_, const char* body_)
{
    int rc = zmq_msg_init (msg_);
    if (rc != 0)
        return -1;
    
    int recv_rc = zmq_msg_recv (msg_, s_, 0);
    if (recv_rc == -1)
        return -1;
    
    if (strcmp (zmq_msg_group (msg_), group_) != 0)
    {
        zmq_msg_close (msg_);
        return -1;
    }
    
    char * body = (char*) malloc (sizeof(char) * (zmq_msg_size (msg_) + 1));
    memcpy (body, zmq_msg_data (msg_), zmq_msg_size (msg_));
    body [zmq_msg_size (msg_)] = '\0';
    
    if (strcmp (body, body_) != 0)
    {
        zmq_msg_close (msg_);
        return -1;
    }
    
    zmq_msg_close (msg_);
    free(body);
    return recv_rc;
}

int test_radio_dish (void)
{
    NSLog(@"- test_radio_dish");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *radio = zmq_socket (ctx, ZMQ_RADIO);
    void *dish = zmq_socket (ctx, ZMQ_DISH);
    
    int rc = zmq_bind (radio, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    //  Leaving a group which we didn't join
    rc = zmq_leave (dish, "Movies");
    assert (rc == -1);
    
    //  Joining too long group
    char too_long_group[ZMQ_GROUP_MAX_LENGTH + 2];
    for (int index = 0; index < ZMQ_GROUP_MAX_LENGTH + 2; index++)
        too_long_group[index] = 'A';
    too_long_group[ZMQ_GROUP_MAX_LENGTH + 1] = '\0';
    rc = zmq_join (dish, too_long_group);
    assert (rc == -1);
    
    // Joining
    rc = zmq_join (dish, "Movies");
    assert (rc == 0);
    
    // Duplicate Joining
    rc = zmq_join (dish, "Movies");
    assert (rc == -1);
    
    // Connecting
    rc = zmq_connect (dish, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    
    zmq_msg_t msg;
    
    //  This is not going to be sent as dish only subscribe to "Movies"
    rc = msg_send2 (&msg, radio, "TV", "Friends");
    assert (rc == 7);
    
    //  This is going to be sent to the dish
    rc = msg_send2 (&msg, radio, "Movies", "Godfather");
    assert (rc == 9);
    
    //  Check the correct message arrived
    rc = msg_recv_cmp2 (&msg, dish, "Movies", "Godfather");
    assert (rc == 9);
    
    //  Join group during connection optvallen
    rc = zmq_join (dish, "TV");
    assert (rc == 0);
    
    zmq_sleep (1);
    
    //  This should arrive now as we joined the group
    rc = msg_send2 (&msg, radio, "TV", "Friends");
    assert (rc == 7);
    
    //  Check the correct message arrived
    rc = msg_recv_cmp2 (&msg, dish, "TV", "Friends");
    assert (rc == 7);
    
    //  Leaving groupr
    rc = zmq_leave (dish, "TV");
    assert (rc == 0);
    
    zmq_sleep (1);
    
    //  This is not going to be sent as dish only subscribe to "Movies"
    rc = msg_send2 (&msg, radio, "TV", "Friends");
    assert (rc == 7);
    
    //  This is going to be sent to the dish
    rc = msg_send2(&msg, radio, "Movies", "Godfather");
    assert (rc == 9);
    
    // test zmq_poll with dish
    zmq_pollitem_t items [] = {
        { radio, 0, ZMQ_POLLIN, 0 }, // read publications
        { dish, 0, ZMQ_POLLIN, 0 }, // read subscriptions
    };
    rc = zmq_poll(items, 2, 2000);
    assert (rc == 1);
    assert (items[1].revents == ZMQ_POLLIN);
    
    //  Check the correct message arrived
    rc = msg_recv_cmp2 (&msg, dish, "Movies", "Godfather");
    assert (rc == 9);
    
    rc = zmq_close (dish);
    assert (rc == 0);
    
    rc = zmq_close (radio);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_req_correlate (void)
{
    NSLog(@"- test_req_correlate");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    int enabled = 1;
    int rc = zmq_setsockopt (req, ZMQ_REQ_CORRELATE, &enabled, sizeof (int));
    assert (rc == 0);
    
    int rcvtimeo = 100;
    rc = zmq_setsockopt (req, ZMQ_RCVTIMEO, &rcvtimeo, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_connect (req, "tcp://localhost:5555");
    assert (rc == 0);
    
    rc = zmq_bind (router, "tcp://127.0.0.1:5555");
    assert (rc == 0);
    
    // Send a multi-part request.
    s_send_seq (req, "ABC", "DEF", SEQ_END);
    
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    // Receive peer identity
    rc = zmq_msg_recv (&msg, router, 0);
    assert (rc != -1);
    assert (zmq_msg_size (&msg) > 0);
    zmq_msg_t peer_id_msg;
    zmq_msg_init (&peer_id_msg);
    zmq_msg_copy (&peer_id_msg, &msg);
    
    int more = 0;
    size_t more_size = sizeof (more);
    rc = zmq_getsockopt (router, ZMQ_RCVMORE, &more, &more_size);
    assert (rc == 0);
    assert (more);
    
    // Receive request id 1
    rc = zmq_msg_recv (&msg, router, 0);
    assert (rc != -1);
    assert (zmq_msg_size (&msg) == sizeof(uint32_t));
    uint32_t req_id = *static_cast<uint32_t *> (zmq_msg_data (&msg));
    zmq_msg_t req_id_msg;
    zmq_msg_init (&req_id_msg);
    zmq_msg_copy (&req_id_msg, &msg);
    
    more = 0;
    more_size = sizeof (more);
    rc = zmq_getsockopt (router, ZMQ_RCVMORE, &more, &more_size);
    assert (rc == 0);
    assert (more);
    
    // Receive the rest.
    s_recv_seq (router, 0, "ABC", "DEF", SEQ_END);
    
    uint32_t bad_req_id = req_id + 1;
    
    // Send back a bad reply: wrong req id, 0, data
    zmq_msg_copy (&msg, &peer_id_msg);
    rc = zmq_msg_send (&msg, router, ZMQ_SNDMORE);
    assert (rc != -1);
    zmq_msg_init_data (&msg, &bad_req_id, sizeof (uint32_t), NULL, NULL);
    rc = zmq_msg_send (&msg, router, ZMQ_SNDMORE);
    assert (rc != -1);
    s_send_seq (router, 0, "DATA", SEQ_END);
    
    // Send back a good reply: good req id, 0, data
    zmq_msg_copy (&msg, &peer_id_msg);
    rc = zmq_msg_send (&msg, router, ZMQ_SNDMORE);
    assert (rc != -1);
    zmq_msg_copy (&msg, &req_id_msg);
    rc = zmq_msg_send (&msg, router, ZMQ_SNDMORE);
    assert (rc != -1);
    s_send_seq (router, 0, "GHI", SEQ_END);
    
    // Receive reply. If bad reply got through, we wouldn't see
    // this particular data.
    s_recv_seq (req, "GHI", SEQ_END);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    rc = zmq_msg_close (&peer_id_msg);
    assert (rc == 0);
    
    rc = zmq_msg_close (&req_id_msg);
    assert (rc == 0);
    
    close_zero_linger (req);
    close_zero_linger (router);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

static void bounce4 (void *socket)
{
    int more;
    size_t more_size = sizeof (more);
    do {
        zmq_msg_t recv_part, sent_part;
        int rc = zmq_msg_init (&recv_part);
        assert (rc == 0);
        
        rc = zmq_msg_recv (&recv_part, socket, 0);
        assert (rc != -1);
        
        rc = zmq_getsockopt (socket, ZMQ_RCVMORE, &more, &more_size);
        assert (rc == 0);
        
        zmq_msg_init (&sent_part);
        zmq_msg_copy (&sent_part, &recv_part);
        
        rc = zmq_msg_send (&sent_part, socket, more ? ZMQ_SNDMORE : 0);
        assert (rc != -1);
        
        zmq_msg_close (&recv_part);
    } while (more);
}

int test_req_relaxed (void)
{
    NSLog(@"- test_req_relaxed");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    int enabled = 1;
    int rc = zmq_setsockopt (req, ZMQ_REQ_RELAXED, &enabled, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_setsockopt (req, ZMQ_REQ_CORRELATE, &enabled, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (req, "tcp://127.0.0.1:5555");
    assert (rc == 0);
    
    const size_t services = 5;
    void *rep [services];
    for (size_t peer = 0; peer < services; peer++) {
        rep [peer] = zmq_socket (ctx, ZMQ_REP);
        assert (rep [peer]);
        
        int timeout = 500;
        rc = zmq_setsockopt (rep [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (rep [peer], "tcp://localhost:5555");
        assert (rc == 0);
    }
    //  We have to give the connects time to finish otherwise the requests
    //  will not properly round-robin. We could alternatively connect the
    //  REQ sockets to the REP sockets.
    msleep (SETTLE_TIME);
    
    //  Case 1: Second send() before a reply arrives in a pipe.
    
    //  Send a request, ensure it arrives, don't send a reply
    s_send_seq (req, "A", "B", SEQ_END);
    s_recv_seq (rep [0], "A", "B", SEQ_END);
    
    //  Send another request on the REQ socket
    s_send_seq (req, "C", "D", SEQ_END);
    s_recv_seq (rep [1], "C", "D", SEQ_END);
    
    //  Send a reply to the first request - that should be discarded by the REQ
    s_send_seq (rep [0], "WRONG", SEQ_END);
    
    //  Send the expected reply
    s_send_seq (rep [1], "OK", SEQ_END);
    s_recv_seq (req, "OK", SEQ_END);
    
    
    //  Another standard req-rep cycle, just to check
    s_send_seq (req, "E", SEQ_END);
    s_recv_seq (rep [2], "E", SEQ_END);
    s_send_seq (rep [2], "F", "G", SEQ_END);
    s_recv_seq (req, "F", "G", SEQ_END);
    
    
    //  Case 2: Second send() after a reply is already in a pipe on the REQ.
    
    //  Send a request, ensure it arrives, send a reply
    s_send_seq (req, "H", SEQ_END);
    s_recv_seq (rep [3], "H", SEQ_END);
    s_send_seq (rep [3], "BAD", SEQ_END);
    
    //  Wait for message to be there.
    msleep (SETTLE_TIME);
    
    //  Without receiving that reply, send another request on the REQ socket
    s_send_seq (req, "I", SEQ_END);
    s_recv_seq (rep [4], "I", SEQ_END);
    
    //  Send the expected reply
    s_send_seq (rep [4], "GOOD", SEQ_END);
    s_recv_seq (req, "GOOD", SEQ_END);
    
    //  Case 3: Check issue #1690. Two send() in a row should not close the
    //  communication pipes. For example pipe from req to rep[0] should not be
    //  closed after executing Case 1. So rep[0] should be the next to receive,
    //  not rep[1].
    s_send_seq (req, "J", SEQ_END);
    s_recv_seq (rep [0], "J", SEQ_END);
    
    close_zero_linger (req);
    for (size_t peer = 0; peer < services; peer++)
        close_zero_linger (rep [peer]);
    
    //  Wait for disconnects.
    msleep (SETTLE_TIME);
    
    //  Case 4: Check issue #1695. As messages may pile up before a responder
    //  is available, we check that responses to messages other than the last
    //  sent one are correctly discarded by the REQ pipe
    
    //  Setup REQ socket as client
    req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    rc = zmq_setsockopt (req, ZMQ_REQ_RELAXED, &enabled, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_setsockopt (req, ZMQ_REQ_CORRELATE, &enabled, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_connect (req, "tcp://localhost:5555");
    assert (rc == 0);
    
    //  Setup ROUTER socket as server but do not bind it just yet
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    //  Send two requests
    s_send_seq (req, "TO_BE_DISCARDED", SEQ_END);
    s_send_seq (req, "TO_BE_ANSWERED", SEQ_END);
    
    //  Bind server allowing it to receive messages
    rc = zmq_bind (router, "tcp://127.0.0.1:5555");
    assert (rc == 0);
    
    //  Read the two messages and send them back as is
    bounce4 (router);
    bounce4 (router);
    
    //  Read the expected correlated reply. As the ZMQ_REQ_CORRELATE is active,
    //  the expected answer is "TO_BE_ANSWERED", not "TO_BE_DISCARDED".
    s_recv_seq (req, "TO_BE_ANSWERED", SEQ_END);
    
    close_zero_linger (req);
    close_zero_linger (router);
    
    //  Wait for disconnects.
    msleep (SETTLE_TIME);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_reqrep_device (void)
{
    NSLog(@"- test_reqrep_device");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create a req/rep device.
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    int rc = zmq_bind (dealer, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    rc = zmq_bind (router, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    //  Create a worker.
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    rc = zmq_connect (rep, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Create a client.
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    rc = zmq_connect (req, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    //  Send a request.
    rc = zmq_send (req, "ABC", 3, ZMQ_SNDMORE);
    assert (rc == 3);
    rc = zmq_send (req, "DEF", 3, 0);
    assert (rc == 3);
    
    //  Pass the request through the device.
    for (int i = 0; i != 4; i++) {
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_msg_recv (&msg, router, 0);
        assert (rc >= 0);
        int rcvmore;
        size_t sz = sizeof (rcvmore);
        rc = zmq_getsockopt (router, ZMQ_RCVMORE, &rcvmore, &sz);
        assert (rc == 0);
        rc = zmq_msg_send (&msg, dealer, rcvmore? ZMQ_SNDMORE: 0);
        assert (rc >= 0);
    }
    
    //  Receive the request.
    char buff [3];
    rc = zmq_recv (rep, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "ABC", 3) == 0);
    int rcvmore;
    size_t sz = sizeof (rcvmore);
    rc = zmq_getsockopt (rep, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (rep, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "DEF", 3) == 0);
    rc = zmq_getsockopt (rep, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
    
    //  Send the reply.
    rc = zmq_send (rep, "GHI", 3, ZMQ_SNDMORE);
    assert (rc == 3);
    rc = zmq_send (rep, "JKL", 3, 0);
    assert (rc == 3);
    
    //  Pass the reply through the device.
    for (int i = 0; i != 4; i++) {
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_msg_recv (&msg, dealer, 0);
        assert (rc >= 0);
        int rcvmore;
        rc = zmq_getsockopt (dealer, ZMQ_RCVMORE, &rcvmore, &sz);
        assert (rc == 0);
        rc = zmq_msg_send (&msg, router, rcvmore? ZMQ_SNDMORE: 0);
        assert (rc >= 0);
    }
    
    //  Receive the reply.
    rc = zmq_recv (req, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "GHI", 3) == 0);
    rc = zmq_getsockopt (req, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (req, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "JKL", 3) == 0);
    rc = zmq_getsockopt (req, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
    
    //  Clean up.
    rc = zmq_close (req);
    assert (rc == 0);
    rc = zmq_close (rep);
    assert (rc == 0);
    rc = zmq_close (router);
    assert (rc == 0);
    rc = zmq_close (dealer);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_reqrep_device_tipc (void)
{
    NSLog(@"- test_reqrep_device_tipc");
    
    void *ctx = zmq_init (1);
    assert (ctx);
    
    //  Create a req/rep device.
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    int rc = zmq_bind (dealer, "tipc://{5560,0,0}");
    assert (rc == 0);
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    rc = zmq_bind (router, "tipc://{5561,0,0}");
    assert (rc == 0);
    
    //  Create a worker.
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    rc = zmq_connect (rep, "tipc://{5560,0}@0.0.0");
    assert (rc == 0);
    
    //  Create a client.
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    rc = zmq_connect (req, "tipc://{5561,0}@0.0.0");
    assert (rc == 0);
    
    //  Send a request.
    rc = zmq_send (req, "ABC", 3, ZMQ_SNDMORE);
    assert (rc == 3);
    rc = zmq_send (req, "DEF", 3, 0);
    assert (rc == 3);
    
    //  Pass the request through the device.
    for (int i = 0; i != 4; i++) {
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_recvmsg (router, &msg, 0);
        assert (rc >= 0);
        int rcvmore;
        size_t sz = sizeof (rcvmore);
        rc = zmq_getsockopt (router, ZMQ_RCVMORE, &rcvmore, &sz);
        assert (rc == 0);
        rc = zmq_sendmsg (dealer, &msg, rcvmore ? ZMQ_SNDMORE : 0);
        assert (rc >= 0);
    }
    
    //  Receive the request.
    char buff [3];
    rc = zmq_recv (rep, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "ABC", 3) == 0);
    int rcvmore;
    size_t sz = sizeof (rcvmore);
    rc = zmq_getsockopt (rep, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (rep, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "DEF", 3) == 0);
    rc = zmq_getsockopt (rep, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
    
    //  Send the reply.
    rc = zmq_send (rep, "GHI", 3, ZMQ_SNDMORE);
    assert (rc == 3);
    rc = zmq_send (rep, "JKL", 3, 0);
    assert (rc == 3);
    
    //  Pass the reply through the device.
    for (int i = 0; i != 4; i++) {
        zmq_msg_t msg;
        rc = zmq_msg_init (&msg);
        assert (rc == 0);
        rc = zmq_recvmsg (dealer, &msg, 0);
        assert (rc >= 0);
        int rcvmore;
        rc = zmq_getsockopt (dealer, ZMQ_RCVMORE, &rcvmore, &sz);
        assert (rc == 0);
        rc = zmq_sendmsg (router, &msg, rcvmore ? ZMQ_SNDMORE : 0);
        assert (rc >= 0);
    }
    
    //  Receive the reply.
    rc = zmq_recv (req, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "GHI", 3) == 0);
    rc = zmq_getsockopt (req, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (rcvmore);
    rc = zmq_recv (req, buff, 3, 0);
    assert (rc == 3);
    assert (memcmp (buff, "JKL", 3) == 0);
    rc = zmq_getsockopt (req, ZMQ_RCVMORE, &rcvmore, &sz);
    assert (rc == 0);
    assert (!rcvmore);
    
    //  Clean up.
    rc = zmq_close (req);
    assert (rc == 0);
    rc = zmq_close (rep);
    assert (rc == 0);
    rc = zmq_close (router);
    assert (rc == 0);
    rc = zmq_close (dealer);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_reqrep_inproc (void)
{
    NSLog(@"- test_reqrep_inproc");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    int rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    rc = zmq_connect (sc, "inproc://a");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

void test_single_connect_ipv4 (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    int rc = zmq_bind (sb, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multi_connect_ipv4 (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb0 = zmq_socket (ctx, ZMQ_REP);
    assert (sb0);
    int rc = zmq_bind (sb0, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *sb1 = zmq_socket (ctx, ZMQ_REP);
    assert (sb1);
    rc = zmq_bind (sb1, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    void *sb2 = zmq_socket (ctx, ZMQ_REP);
    assert (sb2);
    rc = zmq_bind (sb2, "tcp://127.0.0.1:5562");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5564;127.0.0.1:5562");
    assert (rc == 0);
    
    bounce (sb0, sc);
    bounce (sb1, sc);
    bounce (sb2, sc);
    bounce (sb0, sc);
    bounce (sb1, sc);
    bounce (sb2, sc);
    bounce (sb0, sc);
    
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5564;127.0.0.1:5562");
    assert (rc == 0);
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb0, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb1, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb2, "tcp://127.0.0.1:5562");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb0);
    assert (rc == 0);
    
    rc = zmq_close (sb1);
    assert (rc == 0);
    
    rc = zmq_close (sb2);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multi_connect_ipv4_same_port (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb0 = zmq_socket (ctx, ZMQ_REP);
    assert (sb0);
    int rc = zmq_bind (sb0, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *sb1 = zmq_socket (ctx, ZMQ_REP);
    assert (sb1);
    rc = zmq_bind (sb1, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    void *sc0 = zmq_socket (ctx, ZMQ_REQ);
    assert (sc0);
    rc = zmq_connect (sc0, "tcp://127.0.0.1:5564;127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_connect (sc0, "tcp://127.0.0.1:5565;127.0.0.1:5561");
    assert (rc == 0);
    
    void *sc1 = zmq_socket (ctx, ZMQ_REQ);
    assert (sc1);
    rc = zmq_connect (sc1, "tcp://127.0.0.1:5565;127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_connect (sc1, "tcp://127.0.0.1:5564;127.0.0.1:5561");
    assert (rc == 0);
    
    bounce (sb0, sc0);
    bounce (sb1, sc0);
    bounce (sb0, sc1);
    bounce (sb1, sc1);
    bounce (sb0, sc0);
    bounce (sb1, sc0);
    
    rc = zmq_disconnect (sc1, "tcp://127.0.0.1:5565;127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc1, "tcp://127.0.0.1:5564;127.0.0.1:5561");
    assert (rc == 0);
    rc = zmq_disconnect (sc0, "tcp://127.0.0.1:5564;127.0.0.1:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc0, "tcp://127.0.0.1:5565;127.0.0.1:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb0, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb1, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    rc = zmq_close (sc0);
    assert (rc == 0);
    
    rc = zmq_close (sc1);
    assert (rc == 0);
    
    rc = zmq_close (sb0);
    assert (rc == 0);
    
    rc = zmq_close (sb1);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_single_connect_ipv6 (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    if (!is_ipv6_available ()) {
        zmq_ctx_term (ctx);
        return;
    }
    
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    int ipv6 = 1;
    int rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb, "tcp://[::1]:5560");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://[::1]:5560");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, "tcp://[::1]:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb, "tcp://[::1]:5560");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multi_connect_ipv6 (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    if (!is_ipv6_available ()) {
        zmq_ctx_term (ctx);
        return;
    }
    
    void *sb0 = zmq_socket (ctx, ZMQ_REP);
    assert (sb0);
    int ipv6 = 1;
    int rc = zmq_setsockopt (sb0, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb0, "tcp://[::1]:5560");
    assert (rc == 0);
    
    void *sb1 = zmq_socket (ctx, ZMQ_REP);
    assert (sb1);
    rc = zmq_setsockopt (sb1, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb1, "tcp://[::1]:5561");
    assert (rc == 0);
    
    void *sb2 = zmq_socket (ctx, ZMQ_REP);
    assert (sb2);
    rc = zmq_setsockopt (sb2, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb2, "tcp://[::1]:5562");
    assert (rc == 0);
    
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://[::1]:5560");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://[::1]:5561");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://[::1]:5564;[::1]:5562");
    assert (rc == 0);
    
    bounce (sb0, sc);
    bounce (sb1, sc);
    bounce (sb2, sc);
    bounce (sb0, sc);
    bounce (sb1, sc);
    bounce (sb2, sc);
    bounce (sb0, sc);
    
    rc = zmq_disconnect (sc, "tcp://[::1]:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc, "tcp://[::1]:5564;[::1]:5562");
    assert (rc == 0);
    rc = zmq_disconnect (sc, "tcp://[::1]:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb0, "tcp://[::1]:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb1, "tcp://[::1]:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb2, "tcp://[::1]:5562");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    
    rc = zmq_close (sb0);
    assert (rc == 0);
    
    rc = zmq_close (sb1);
    assert (rc == 0);
    
    rc = zmq_close (sb2);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_multi_connect_ipv6_same_port (void)
{
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    if (!is_ipv6_available ()) {
        zmq_ctx_term (ctx);
        return;
    }
    
    void *sb0 = zmq_socket (ctx, ZMQ_REP);
    assert (sb0);
    int ipv6 = 1;
    int rc = zmq_setsockopt (sb0, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb0, "tcp://[::1]:5560");
    assert (rc == 0);
    
    void *sb1 = zmq_socket (ctx, ZMQ_REP);
    assert (sb1);
    rc = zmq_setsockopt (sb1, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (sb1, "tcp://[::1]:5561");
    assert (rc == 0);
    
    void *sc0 = zmq_socket (ctx, ZMQ_REQ);
    assert (sc0);
    rc = zmq_setsockopt (sc0, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_connect (sc0, "tcp://[::1]:5564;[::1]:5560");
    assert (rc == 0);
    rc = zmq_connect (sc0, "tcp://[::1]:5565;[::1]:5561");
    assert (rc == 0);
    
    void *sc1 = zmq_socket (ctx, ZMQ_REQ);
    assert (sc1);
    rc = zmq_setsockopt (sc1, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_connect (sc1, "tcp://[::1]:5565;[::1]:5560");
    assert (rc == 0);
    rc = zmq_connect (sc1, "tcp://[::1]:5564;[::1]:5561");
    assert (rc == 0);
    
    bounce (sb0, sc0);
    bounce (sb1, sc0);
    bounce (sb0, sc1);
    bounce (sb1, sc1);
    bounce (sb0, sc0);
    bounce (sb1, sc0);
    
    rc = zmq_disconnect (sc1, "tcp://[::1]:5565;[::1]:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc1, "tcp://[::1]:5564;[::1]:5561");
    assert (rc == 0);
    rc = zmq_disconnect (sc0, "tcp://[::1]:5564;[::1]:5560");
    assert (rc == 0);
    rc = zmq_disconnect (sc0, "tcp://[::1]:5565;[::1]:5561");
    assert (rc == 0);
    
    rc = zmq_unbind (sb0, "tcp://[::1]:5560");
    assert (rc == 0);
    
    rc = zmq_unbind (sb1, "tcp://[::1]:5561");
    assert (rc == 0);
    
    rc = zmq_close (sc0);
    assert (rc == 0);
    
    rc = zmq_close (sc1);
    assert (rc == 0);
    
    rc = zmq_close (sb0);
    assert (rc == 0);
    
    rc = zmq_close (sb1);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

int test_reqrep_tcp (void)
{
    NSLog(@"- test_reqrep_tcp");
    setup_test_environment ();
    
    test_single_connect_ipv4 ();
    
    test_multi_connect_ipv4 ();
    
    test_multi_connect_ipv4_same_port ();
    
    test_single_connect_ipv6 ();
    
    test_multi_connect_ipv6 ();
    
    test_multi_connect_ipv6_same_port ();
    
    return 0 ;
}

int test_router_handover (void)
{
    NSLog(@"- test_router_handover");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    int rc = zmq_bind (router, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    // Enable the handover flag
    int handover = 1;
    rc = zmq_setsockopt (router, ZMQ_ROUTER_HANDOVER, &handover, sizeof (handover));
    assert (rc == 0);
    
    //  Create dealer called "X" and connect it to our router
    void *dealer_one = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer_one);
    rc = zmq_setsockopt (dealer_one, ZMQ_IDENTITY, "X", 1);
    assert (rc == 0);
    rc = zmq_connect (dealer_one, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Get message from dealer to know when connection is ready
    char buffer [255];
    rc = zmq_send (dealer_one, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 1);
    assert (buffer [0] ==  'X');
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 5);
    
    // Now create a second dealer that uses the same identity
    void *dealer_two = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer_two);
    rc = zmq_setsockopt (dealer_two, ZMQ_IDENTITY, "X", 1);
    assert (rc == 0);
    rc = zmq_connect (dealer_two, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Get message from dealer to know when connection is ready
    rc = zmq_send (dealer_two, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 1);
    assert (buffer [0] ==  'X');
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 5);
    
    //  Send a message to 'X' identity. This should be delivered
    //  to the second dealer, instead of the first beccause of the handover.
    rc = zmq_send (router, "X", 1, ZMQ_SNDMORE);
    assert (rc == 1);
    rc = zmq_send (router, "Hello", 5, 0);
    assert (rc == 5);
    
    //  Ensure that the first dealer doesn't receive the message
    //  but the second one does
    rc = zmq_recv (dealer_one, buffer, 255, ZMQ_NOBLOCK);
    assert (rc == -1);
    
    rc = zmq_recv (dealer_two, buffer, 255, 0);
    assert (rc == 5);
    
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_close (dealer_one);
    assert (rc == 0);
    
    rc = zmq_close (dealer_two);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_router_mandatory (void)
{
    NSLog(@"- test_router_mandatory");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    int rc = zmq_bind (router, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Send a message to an unknown peer with the default setting
    //  This will not report any error
    rc = zmq_send (router, "UNKNOWN", 7, ZMQ_SNDMORE);
    assert (rc == 7);
    rc = zmq_send (router, "DATA", 4, 0);
    assert (rc == 4);
    
    //  Send a message to an unknown peer with mandatory routing
    //  This will fail
    int mandatory = 1;
    rc = zmq_setsockopt (router, ZMQ_ROUTER_MANDATORY, &mandatory, sizeof (mandatory));
    assert (rc == 0);
    rc = zmq_send (router, "UNKNOWN", 7, ZMQ_SNDMORE);
    assert (rc == -1 && errno == EHOSTUNREACH);
    
    //  Create dealer called "X" and connect it to our router
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    rc = zmq_setsockopt (dealer, ZMQ_IDENTITY, "X", 1);
    assert (rc == 0);
    rc = zmq_connect (dealer, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Get message from dealer to know when connection is ready
    char buffer [255];
    rc = zmq_send (dealer, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 1);
    assert (buffer [0] ==  'X');
    
    //  Send a message to connected dealer now
    //  It should work
    rc = zmq_send (router, "X", 1, ZMQ_SNDMORE);
    assert (rc == 1);
    rc = zmq_send (router, "Hello", 5, 0);
    assert (rc == 5);
    
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_router_mandatory_hwn (void)
{
    NSLog(@"- test_router_mandatory_hwn");
    int rc;
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    // Configure router socket to mandatory routing and set HWM and linger
    int mandatory = 1;
    rc = zmq_setsockopt (router, ZMQ_ROUTER_MANDATORY, &mandatory, sizeof (mandatory));
    assert (rc == 0);
    int sndhwm = 1;
    rc = zmq_setsockopt (router, ZMQ_SNDHWM, &sndhwm, sizeof (sndhwm));
    assert (rc == 0);
    int linger = 1;
    rc = zmq_setsockopt (router, ZMQ_LINGER, &linger, sizeof (linger));
    assert (rc == 0);
    
    rc = zmq_bind (router, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Create dealer called "X" and connect it to our router, configure HWM
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    rc = zmq_setsockopt (dealer, ZMQ_IDENTITY, "X", 1);
    assert (rc == 0);
    int rcvhwm = 1;
    rc = zmq_setsockopt (dealer, ZMQ_RCVHWM, &rcvhwm, sizeof (rcvhwm));
    assert (rc == 0);
    
    rc = zmq_connect (dealer, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Get message from dealer to know when connection is ready
    char buffer [255];
    rc = zmq_send (dealer, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (router, buffer, 255, 0);
    assert (rc == 1);
    assert (buffer [0] ==  'X');
    
    int i;
    const int BUF_SIZE = 65536;
    char buf[BUF_SIZE];
    memset(buf, 0, BUF_SIZE);
    // Send first batch of messages
    for(i = 0; i < 100000; ++i) {
        rc = zmq_send (router, "X", 1, ZMQ_DONTWAIT | ZMQ_SNDMORE);
        if (rc == -1 && zmq_errno() == EAGAIN) break;
        assert (rc == 1);
        rc = zmq_send (router, buf, BUF_SIZE, ZMQ_DONTWAIT);
        assert (rc == BUF_SIZE);
    }
    // This should fail after one message but kernel buffering could
    // skew results
    assert (i < 10);
    msleep (1000);
    // Send second batch of messages
    for(; i < 100000; ++i) {
        rc = zmq_send (router, "X", 1, ZMQ_DONTWAIT | ZMQ_SNDMORE);
        if (rc == -1 && zmq_errno() == EAGAIN) break;
        assert (rc == 1);
        rc = zmq_send (router, buf, BUF_SIZE, ZMQ_DONTWAIT);
        assert (rc == BUF_SIZE);
    }
    // This should fail after two messages but kernel buffering could
    // skew results
    assert (i < 20);
    
    
    rc = zmq_close (router);
    assert (rc == 0);
    
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_scatter_gather (void)
{
    NSLog(@"- test_scatter_gather");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *scatter = zmq_socket (ctx, ZMQ_SCATTER);
    void *gather = zmq_socket (ctx, ZMQ_GATHER);
    void *gather2 = zmq_socket (ctx, ZMQ_GATHER);
    
    int rc = zmq_bind (scatter, "inproc://test-scatter-gather");
    assert (rc == 0);
    
    rc = zmq_connect (gather, "inproc://test-scatter-gather");
    assert (rc == 0);
    
    rc = zmq_connect (gather2, "inproc://test-scatter-gather");
    assert (rc == 0);
    
    //  Should fail, multipart is not supported
    rc = s_sendmore (scatter, "1");
    assert (rc == -1);
    
    rc = s_send (scatter, "1");
    assert (rc == 1);
    
    rc = s_send (scatter, "2");
    assert (rc == 1);
    
    char* message = s_recv (gather);
    assert (message);
    assert (streq(message, "1"));
    free(message);
    
    message = s_recv (gather2);
    assert (message);
    assert (streq(message, "2"));
    free(message);
    
    rc = zmq_close (scatter);
    assert (rc == 0);
    
    rc = zmq_close (gather);
    assert (rc == 0);
    
    rc = zmq_close (gather2);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


//  We'll generate random test keys at startup
static char client_public [41];
static char client_secret [41];
static char server_public [41];
static char server_secret [41];

#ifdef ZMQ_BUILD_DRAFT_API
//  Read one event off the monitor socket; return value and address
//  by reference, if not null, and event number by value. Returns -1
//  in case of error.

static int
get_monitor_event5 (void *monitor, int *value, char **address)
{
    //  First frame in message contains event number and value
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (zmq_msg_more (&msg));
    
    uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
    uint16_t event = *(uint16_t *) (data);
    if (value)
        *value = *(uint32_t *) (data + 2);
    
    //  Second frame in message contains event address
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (!zmq_msg_more (&msg));
    
    if (address) {
        uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
        size_t size = zmq_msg_size (&msg);
        *address = (char *) malloc (size + 1);
        memcpy (*address, data, size);
        *address [size] = 0;
    }
    return event;
}
#endif


//  --------------------------------------------------------------------------
//  This methods receives and validates ZAP requestes (allowing or denying
//  each client connection).

static void zap_handler5 (void *handler)
{
    //  Process ZAP requests forever
    while (true) {
        char *version = s_recv (handler);
        if (!version)
            break;          //  Terminating
        
        char *sequence = s_recv (handler);
        char *domain = s_recv (handler);
        char *address = s_recv (handler);
        char *identity = s_recv (handler);
        char *mechanism = s_recv (handler);
        uint8_t client_key [32];
        int size = zmq_recv (handler, client_key, 32, 0);
        assert (size == 32);
        
        char client_key_text [41];
        zmq_z85_encode (client_key_text, client_key, 32);
        
        assert (streq (version, "1.0"));
        assert (streq (mechanism, "CURVE"));
        assert (streq (identity, "IDENT"));
        
        s_sendmore (handler, version);
        s_sendmore (handler, sequence);
        
        if (streq (client_key_text, client_public)) {
            s_sendmore (handler, "200");
            s_sendmore (handler, "OK");
            s_sendmore (handler, "anonymous");
            s_send     (handler, "");
        }
        else {
            s_sendmore (handler, "400");
            s_sendmore (handler, "Invalid client public key");
            s_sendmore (handler, "");
            s_send     (handler, "");
        }
        free (version);
        free (sequence);
        free (domain);
        free (address);
        free (identity);
        free (mechanism);
    }
    zmq_close (handler);
}


int test_security_curve (void)
{
    NSLog(@"- test_security_curve");
#ifndef ZMQ_HAVE_CURVE
    NSLog(@"- test_security_curve: CURVE encryption not installed, skipping test");
    return 0;
#endif
    //  Generate new keypairs for this test
    int rc = zmq_curve_keypair (client_public, client_secret);
    assert (rc == 0);
    rc = zmq_curve_keypair (server_public, server_secret);
    assert (rc == 0);
    
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Spawn ZAP handler
    //  We create and bind ZAP socket in main thread to avoid case
    //  where child thread does not start up fast enough.
    void *handler = zmq_socket (ctx, ZMQ_REP);
    assert (handler);
    rc = zmq_bind (handler, "inproc://zeromq.zap.01");
    assert (rc == 0);
    void *zap_thread = zmq_threadstart (&zap_handler5, handler);
    
    //  Server socket will accept connections
    void *server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    int as_server = 1;
    rc = zmq_setsockopt (server, ZMQ_CURVE_SERVER, &as_server, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (server, ZMQ_CURVE_SECRETKEY, server_secret, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (server, ZMQ_IDENTITY, "IDENT", 6);
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9998");
    assert (rc == 0);
    
#ifdef ZMQ_BUILD_DRAFT_API
    //  Monitor handshake events on the server
    rc = zmq_socket_monitor (server, "inproc://monitor-server",
                             ZMQ_EVENT_HANDSHAKE_SUCCEED | ZMQ_EVENT_HANDSHAKE_FAILED);
    assert (rc == 0);
    
    //  Create socket for collecting monitor events
    void *server_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (server_mon);
    
    //  Connect it to the inproc endpoints so they'll get events
    rc = zmq_connect (server_mon, "inproc://monitor-server");
    assert (rc == 0);
#endif
    
    //  Check CURVE security with valid credentials
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, server_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, client_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, client_secret, 41);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    bounce (server, client);
    rc = zmq_close (client);
    assert (rc == 0);
    
#ifdef ZMQ_BUILD_DRAFT_API
    int event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_SUCCEED);
#endif
    
    //  Check CURVE security with a garbage server key
    //  This will be caught by the curve_server class, not passed to ZAP
    char garbage_key [] = "0000000000000000000000000000000000000000";
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, garbage_key, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, client_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, client_secret, 41);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_FAILED);
#endif
    
    //  Check CURVE security with a garbage client public key
    //  This will be caught by the curve_server class, not passed to ZAP
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, server_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, garbage_key, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, client_secret, 41);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_FAILED);
#endif
    
    //  Check CURVE security with a garbage client secret key
    //  This will be caught by the curve_server class, not passed to ZAP
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, server_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, client_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, garbage_key, 41);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_FAILED);
#endif
    
    //  Check CURVE security with bogus client credentials
    //  This must be caught by the ZAP handler
    char bogus_public [41];
    char bogus_secret [41];
    zmq_curve_keypair (bogus_public, bogus_secret);
    
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, server_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, bogus_public, 41);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, bogus_secret, 41);
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_FAILED);
#endif
    
    //  Check CURVE security with NULL client credentials
    //  This must be caught by the curve_server class, not passed to ZAP
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
#ifdef ZMQ_BUILD_DRAFT_API
    event = get_monitor_event5 (server_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_HANDSHAKE_FAILED);
#endif
    
    //  Check CURVE security with PLAIN client credentials
    //  This must be caught by the curve_server class, not passed to ZAP
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_PLAIN_USERNAME, "admin", 5);
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_PLAIN_PASSWORD, "password", 8);
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
    // Unauthenticated messages from a vanilla socket shouldn't be received
    struct sockaddr_in ip4addr;
    int s;
    
    ip4addr.sin_family = AF_INET;
    ip4addr.sin_port = htons (9998);
#if defined (ZMQ_HAVE_WINDOWS) && (_WIN32_WINNT < 0x0600)
    ip4addr.sin_addr.s_addr = inet_addr ("127.0.0.1");
#else
    inet_pton(AF_INET, "127.0.0.1", &ip4addr.sin_addr);
#endif
    
    s = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
    rc = connect (s, (struct sockaddr*) &ip4addr, sizeof (ip4addr));
    assert (rc > -1);
    // send anonymous ZMTP/1.0 greeting
    send (s, "\x01\x00", 2, 0);
    // send sneaky message that shouldn't be received
    send (s, "\x08\x00sneaky\0", 9, 0);
    int timeout = 250;
    zmq_setsockopt (server, ZMQ_RCVTIMEO, &timeout, sizeof (timeout));
    char *buf = s_recv (server);
    if (buf != NULL) {
        printf ("Received unauthenticated message: %s\n", buf);
        assert (buf == NULL);
    }
    close (s);
    
    //  Check return codes for invalid buffer sizes
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    errno = 0;
    rc = zmq_setsockopt (client, ZMQ_CURVE_SERVERKEY, server_public, 123);
    assert (rc == -1 && errno == EINVAL);
    errno = 0;
    rc = zmq_setsockopt (client, ZMQ_CURVE_PUBLICKEY, client_public, 123);
    assert (rc == -1 && errno == EINVAL);
    errno = 0;
    rc = zmq_setsockopt (client, ZMQ_CURVE_SECRETKEY, client_secret, 123);
    assert (rc == -1 && errno == EINVAL);
    rc = zmq_close (client);
    assert (rc == 0);
    
    //  Shutdown
#ifdef ZMQ_BUILD_DRAFT_API
    close_zero_linger (server_mon);
#endif
    rc = zmq_close (server);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Wait until ZAP handler terminates
    zmq_threadclose (zap_thread);
    
    return 0;
}

static void
zap_handler7 (void *handler)
{
    //  Process ZAP requests forever
    while (true) {
        char *version = s_recv (handler);
        if (!version)
            break;          //  Terminating
        
        char *sequence = s_recv (handler);
        char *domain = s_recv (handler);
        char *address = s_recv (handler);
        char *identity = s_recv (handler);
        char *mechanism = s_recv (handler);
        
        assert (streq (version, "1.0"));
        assert (streq (mechanism, "NULL"));
        
        s_sendmore (handler, version);
        s_sendmore (handler, sequence);
        if (streq (domain, "TEST")) {
            s_sendmore (handler, "200");
            s_sendmore (handler, "OK");
            s_sendmore (handler, "anonymous");
            s_send     (handler, "");
        }
        else {
            s_sendmore (handler, "400");
            s_sendmore (handler, "BAD DOMAIN");
            s_sendmore (handler, "");
            s_send     (handler, "");
        }
        free (version);
        free (sequence);
        free (domain);
        free (address);
        free (identity);
        free (mechanism);
    }
    close_zero_linger (handler);
}

int test_sequrity_null (void)
{
    NSLog(@"- test_sequrity_null");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Spawn ZAP handler
    //  We create and bind ZAP socket in main thread to avoid case
    //  where child thread does not start up fast enough.
    void *handler = zmq_socket (ctx, ZMQ_REP);
    assert (handler);
    int rc = zmq_bind (handler, "inproc://zeromq.zap.01");
    assert (rc == 0);
    void *zap_thread = zmq_threadstart (&zap_handler7, handler);
    
    //  We bounce between a binding server and a connecting client
    
    //  We first test client/server with no ZAP domain
    //  Libzmq does not call our ZAP handler, the connect must succeed
    void *server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_bind (server, "tcp://127.0.0.1:9000");
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:9000");
    assert (rc == 0);
    bounce (server, client);
    close_zero_linger (client);
    close_zero_linger (server);
    
    //  Now define a ZAP domain for the server; this enables
    //  authentication. We're using the wrong domain so this test
    //  must fail.
    server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (server, ZMQ_ZAP_DOMAIN, "WRONG", 5);
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9001");
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:9001");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    close_zero_linger (server);
    
    //  Now use the right domain, the test must pass
    server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    rc = zmq_setsockopt (server, ZMQ_ZAP_DOMAIN, "TEST", 4);
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9002");
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://127.0.0.1:9002");
    assert (rc == 0);
    bounce (server, client);
    close_zero_linger (client);
    close_zero_linger (server);
    
    // Unauthenticated messages from a vanilla socket shouldn't be received
    server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    rc = zmq_setsockopt (server, ZMQ_ZAP_DOMAIN, "WRONG", 5);
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9003");
    assert (rc == 0);
    
    struct sockaddr_in ip4addr;
    int s;
    
    ip4addr.sin_family = AF_INET;
    ip4addr.sin_port = htons(9003);
#if defined (ZMQ_HAVE_WINDOWS) && (_WIN32_WINNT < 0x0600)
    ip4addr.sin_addr.s_addr = inet_addr ("127.0.0.1");
#else
    inet_pton(AF_INET, "127.0.0.1", &ip4addr.sin_addr);
#endif
    
    s = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
    rc = connect (s, (struct sockaddr*) &ip4addr, sizeof ip4addr);
    assert (rc > -1);
    // send anonymous ZMTP/1.0 greeting
    send (s, "\x01\x00", 2, 0);
    // send sneaky message that shouldn't be received
    send (s, "\x08\x00sneaky\0", 9, 0);
    int timeout = 250;
    zmq_setsockopt (server, ZMQ_RCVTIMEO, &timeout, sizeof (timeout));
    char *buf = s_recv (server);
    if (buf != NULL) {
        printf ("Received unauthenticated message: %s\n", buf);
        assert (buf == NULL);
    }
    close (s);
    close_zero_linger (server);
    
    //  Shutdown
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    //  Wait until ZAP handler terminates
    zmq_threadclose (zap_thread);
    
    return 0;
}


static void
zap_handler8 (void *ctx)
{
    //  Create and bind ZAP socket
    void *zap = zmq_socket (ctx, ZMQ_REP);
    assert (zap);
    int rc = zmq_bind (zap, "inproc://zeromq.zap.01");
    assert (rc == 0);
    
    //  Process ZAP requests forever
    while (true) {
        char *version = s_recv (zap);
        if (!version)
            break;          //  Terminating
        char *sequence = s_recv (zap);
        char *domain = s_recv (zap);
        char *address = s_recv (zap);
        char *identity = s_recv (zap);
        char *mechanism = s_recv (zap);
        char *username = s_recv (zap);
        char *password = s_recv (zap);
        
        assert (streq (version, "1.0"));
        assert (streq (mechanism, "PLAIN"));
        assert (streq (identity, "IDENT"));
        
        s_sendmore (zap, version);
        s_sendmore (zap, sequence);
        if (streq (username, "admin")
            &&  streq (password, "password")) {
            s_sendmore (zap, "200");
            s_sendmore (zap, "OK");
            s_sendmore (zap, "anonymous");
            s_send (zap, "");
        }
        else {
            s_sendmore (zap, "400");
            s_sendmore (zap, "Invalid username or password");
            s_sendmore (zap, "");
            s_send (zap, "");
        }
        free (version);
        free (sequence);
        free (domain);
        free (address);
        free (identity);
        free (mechanism);
        free (username);
        free (password);
    }
    rc = zmq_close (zap);
    assert (rc == 0);
}

int test_security_plain (void)
{
    NSLog(@"- test_security_plain");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Spawn ZAP handler
    void *zap_thread = zmq_threadstart (&zap_handler8, ctx);
    
    //  Server socket will accept connections
    void *server = zmq_socket (ctx, ZMQ_DEALER);
    assert (server);
    int rc = zmq_setsockopt (server, ZMQ_IDENTITY, "IDENT", 6);
    assert (rc == 0);
    int as_server = 1;
    rc = zmq_setsockopt (server, ZMQ_PLAIN_SERVER, &as_server, sizeof (int));
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9998");
    assert (rc == 0);
    
    char username [256];
    char password [256];
    
    //  Check PLAIN security with correct username/password
    void *client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    strcpy (username, "admin");
    rc = zmq_setsockopt (client, ZMQ_PLAIN_USERNAME, username, strlen (username));
    assert (rc == 0);
    strcpy (password, "password");
    rc = zmq_setsockopt (client, ZMQ_PLAIN_PASSWORD, password, strlen (password));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    bounce (server, client);
    rc = zmq_close (client);
    assert (rc == 0);
    
    //  Check PLAIN security with badly configured client (as_server)
    //  This will be caught by the plain_server class, not passed to ZAP
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    as_server = 1;
    rc = zmq_setsockopt (client, ZMQ_PLAIN_SERVER, &as_server, sizeof (int));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
    //  Check PLAIN security -- failed authentication
    client = zmq_socket (ctx, ZMQ_DEALER);
    assert (client);
    strcpy (username, "wronguser");
    strcpy (password, "wrongpass");
    rc = zmq_setsockopt (client, ZMQ_PLAIN_USERNAME, username, strlen (username));
    assert (rc == 0);
    rc = zmq_setsockopt (client, ZMQ_PLAIN_PASSWORD, password, strlen (password));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9998");
    assert (rc == 0);
    expect_bounce_fail (server, client);
    close_zero_linger (client);
    
    // Unauthenticated messages from a vanilla socket shouldn't be received
    struct sockaddr_in ip4addr;
    int s;
    
    ip4addr.sin_family = AF_INET;
    ip4addr.sin_port = htons (9998);
#if defined (ZMQ_HAVE_WINDOWS) && (_WIN32_WINNT < 0x0600)
    ip4addr.sin_addr.s_addr = inet_addr ("127.0.0.1");
#else
    inet_pton (AF_INET, "127.0.0.1", &ip4addr.sin_addr);
#endif
    
    s = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
    rc = connect (s, (struct sockaddr*) &ip4addr, sizeof (ip4addr));
    assert (rc > -1);
    // send anonymous ZMTP/1.0 greeting
    send (s, "\x01\x00", 2, 0);
    // send sneaky message that shouldn't be received
    send (s, "\x08\x00sneaky\0", 9, 0);
    int timeout = 250;
    zmq_setsockopt (server, ZMQ_RCVTIMEO, &timeout, sizeof (timeout));
    char *buf = s_recv (server);
    if (buf != NULL) {
        printf ("Received unauthenticated message: %s\n", buf);
        assert (buf == NULL);
    }
    close (s);
    
    //  Shutdown
    rc = zmq_close (server);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Wait until ZAP handler terminates
    zmq_threadclose (zap_thread);
    
    return 0;
}

void test_setsockopt_tcp_recv_buffer (void)
{
    int rc;
    void *ctx = zmq_ctx_new ();
    void *socket = zmq_socket (ctx, ZMQ_PUSH);
    
    int val = 0;
    size_t placeholder = sizeof (val);
    
    rc = zmq_getsockopt (socket, ZMQ_RCVBUF, &val, &placeholder);
    assert (rc == 0);
    assert (val == -1);
    
    val = 16384;
    
    rc = zmq_setsockopt (socket, ZMQ_RCVBUF, &val, sizeof (val));
    assert (rc == 0);
    assert (val == 16384);
    
    rc = zmq_getsockopt (socket, ZMQ_RCVBUF, &val, &placeholder);
    assert (rc == 0);
    assert (val == 16384);
    
    zmq_close (socket);
    zmq_ctx_term (ctx);
}

void test_setsockopt_tcp_send_buffer (void)
{
    int rc;
    void *ctx = zmq_ctx_new ();
    void *socket = zmq_socket (ctx, ZMQ_PUSH);
    
    int val = 0;
    size_t placeholder = sizeof (val);
    
    rc = zmq_getsockopt (socket, ZMQ_SNDBUF, &val, &placeholder);
    assert (rc == 0);
    assert (val == -1);
    
    val = 16384;
    
    rc = zmq_setsockopt (socket, ZMQ_SNDBUF, &val, sizeof (val));
    assert (rc == 0);
    assert (val == 16384);
    
    rc = zmq_getsockopt (socket, ZMQ_SNDBUF, &val, &placeholder);
    assert (rc == 0);
    assert (val == 16384);
    
    zmq_close (socket);
    zmq_ctx_term (ctx);
}

void test_setsockopt_use_fd ()
{
    int rc;
    void *ctx = zmq_ctx_new ();
    void *socket = zmq_socket (ctx, ZMQ_PUSH);
    
    int val = 0;
    size_t placeholder = sizeof (val);
    
    rc = zmq_getsockopt (socket, ZMQ_USE_FD, &val, &placeholder);
    assert(rc == 0);
    assert(val == -1);
    
    val = 3;
    
    rc = zmq_setsockopt (socket, ZMQ_USE_FD, &val, sizeof(val));
    assert(rc == 0);
    assert(val == 3);
    
    rc = zmq_getsockopt (socket, ZMQ_USE_FD, &val, &placeholder);
    assert(rc == 0);
    assert(val == 3);
    
    zmq_close (socket);
    zmq_ctx_term (ctx);
}

int test_setsockopt (void)
{
    NSLog(@"- test_setsockopt");
    test_setsockopt_tcp_recv_buffer ();
    test_setsockopt_tcp_send_buffer ();
    test_setsockopt_use_fd ();
    return 0;
}

#define THREAD_COUNT 40

extern "C"
{
    static void worker_stress (void *s)
    {
        int rc;
        
        rc = zmq_connect (s, "tcp://127.0.0.1:5560");
        assert (rc == 0);
        
        //  Start closing the socket while the connecting process is underway.
        rc = zmq_close (s);
        assert (rc == 0);
    }
}

int test_shutdown_stress (void)
{
    NSLog(@"- test_shutdown_stress");
    setup_test_environment();
    void *s1;
    void *s2;
    int i;
    int j;
    int rc;
    void* threads [THREAD_COUNT];
    
    for (j = 0; j != 10; j++) {
        
        //  Check the shutdown with many parallel I/O threads.
        void *ctx = zmq_ctx_new ();
        assert (ctx);
        zmq_ctx_set (ctx, ZMQ_IO_THREADS, 7);
        
        s1 = zmq_socket (ctx, ZMQ_PUB);
        assert (s1);
        
        rc = zmq_bind (s1, "tcp://127.0.0.1:5560");
        assert (rc == 0);
        
        for (i = 0; i != THREAD_COUNT; i++) {
            s2 = zmq_socket (ctx, ZMQ_SUB);
            assert (s2);
            threads [i] = zmq_threadstart(&worker_stress, s2);
        }
        
        for (i = 0; i != THREAD_COUNT; i++) {
            zmq_threadclose(threads [i]);
        }
        
        rc = zmq_close (s1);
        assert (rc == 0);
        
        rc = zmq_ctx_term (ctx);
        assert (rc == 0);
    }
    
    return 0;
}

const int MAX_SENDS2 = 10000;

void test_change_before_connected()
{
    int rc;
    void *ctx = zmq_ctx_new();
    
    void *bind_socket = zmq_socket(ctx, ZMQ_PUSH);
    void *connect_socket = zmq_socket(ctx, ZMQ_PULL);
    
    int val = 2;
    rc = zmq_setsockopt(connect_socket, ZMQ_RCVHWM, &val, sizeof(val));
    assert(rc == 0);
    rc = zmq_setsockopt(bind_socket, ZMQ_SNDHWM, &val, sizeof(val));
    assert(rc == 0);
    
    zmq_connect(connect_socket, "inproc://a");
    zmq_bind(bind_socket, "inproc://a");
    
    size_t placeholder = sizeof(val);
    val = 0;
    rc = zmq_getsockopt(bind_socket, ZMQ_SNDHWM, &val, &placeholder);
    assert(rc == 0);
    assert(val == 2);
    
    int send_count = 0;
    while (send_count < MAX_SENDS2 && zmq_send(bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    assert(send_count == 4);
    
    zmq_close(bind_socket);
    zmq_close(connect_socket);
    zmq_ctx_term(ctx);
}

void test_change_after_connected()
{
    int rc;
    void *ctx = zmq_ctx_new();
    
    void *bind_socket = zmq_socket(ctx, ZMQ_PUSH);
    void *connect_socket = zmq_socket(ctx, ZMQ_PULL);
    
    int val = 1;
    rc = zmq_setsockopt(connect_socket, ZMQ_RCVHWM, &val, sizeof(val));
    assert(rc == 0);
    rc = zmq_setsockopt(bind_socket, ZMQ_SNDHWM, &val, sizeof(val));
    assert(rc == 0);
    
    zmq_connect(connect_socket, "inproc://a");
    zmq_bind(bind_socket, "inproc://a");
    
    val = 5;
    rc = zmq_setsockopt(bind_socket, ZMQ_SNDHWM, &val, sizeof(val));
    assert(rc == 0);
    
    size_t placeholder = sizeof(val);
    val = 0;
    rc = zmq_getsockopt(bind_socket, ZMQ_SNDHWM, &val, &placeholder);
    assert(rc == 0);
    assert(val == 5);
    
    int send_count = 0;
    while (send_count < MAX_SENDS2 && zmq_send(bind_socket, NULL, 0, ZMQ_DONTWAIT) == 0)
        ++send_count;
    
    assert(send_count == 6);
    
    zmq_close(bind_socket);
    zmq_close(connect_socket);
    zmq_ctx_term(ctx);
}

void test_decrease_when_full()
{
    int rc;
    void *ctx = zmq_ctx_new();
    
    void *bind_socket = zmq_socket(ctx, ZMQ_PUSH);
    void *connect_socket = zmq_socket(ctx, ZMQ_PULL);
    
    int val = 1;
    rc = zmq_setsockopt(connect_socket, ZMQ_RCVHWM, &val, sizeof(val));
    assert(rc == 0);
    
    val = 100;
    rc = zmq_setsockopt(bind_socket, ZMQ_SNDHWM, &val, sizeof(val));
    assert(rc == 0);
    
    zmq_bind(bind_socket, "inproc://a");
    zmq_connect(connect_socket, "inproc://a");
    
    // Fill up to hwm
    int send_count = 0;
    while (send_count < MAX_SENDS2 && zmq_send(bind_socket, &send_count, sizeof(send_count), ZMQ_DONTWAIT) == sizeof(send_count))
        ++send_count;
    assert(send_count == 101);
    
    // Descrease snd hwm
    val = 70;
    rc = zmq_setsockopt(bind_socket, ZMQ_SNDHWM, &val, sizeof(val));
    assert(rc == 0);
    
    size_t placeholder = sizeof(val);
    val = 0;
    rc = zmq_getsockopt(bind_socket, ZMQ_SNDHWM, &val, &placeholder);
    assert(rc == 0);
    assert(val == 70);
    
    // Read out all data (should get up to previous hwm worth so none were dropped)
    int read_count = 0;
    int read_data = 0;
    while (read_count < MAX_SENDS2 && zmq_recv(connect_socket, &read_data, sizeof(read_data), ZMQ_DONTWAIT) == sizeof(read_data)) {
        assert(read_count == read_data);
        ++read_count;
    }
    
    assert(read_count == 101);
    
    // Give io thread some time to catch up
    msleep (SETTLE_TIME);
    
    // Fill up to new hwm
    send_count = 0;
    while (send_count < MAX_SENDS2 && zmq_send(bind_socket, &send_count, sizeof(send_count), ZMQ_DONTWAIT) == sizeof(send_count))
        ++send_count;
    
    // Really this should be 71, but the lwm stuff kicks in doesn't seem quite right
    assert(send_count > 0);
    
    zmq_close(bind_socket);
    zmq_close(connect_socket);
    zmq_ctx_term(ctx);
}


void test_sockopt_hwm()
{
    NSLog(@"- test_sockopt_hwm");
    test_change_before_connected();
    test_change_after_connected();
    test_decrease_when_full();
}

// There is no way to test for correctness because of the embedded RNG.
void test__zmq_curve_keypair__always__success (void)
{
    errno = 0;
    char public_key[41] = { 0 };
    char secret_key[41] = { 0 };
    
    const int rc = zmq_curve_keypair(public_key, secret_key);
    
#if defined (ZMQ_HAVE_CURVE)
    assert (rc == 0);
#else
    assert (rc == -1);
    assert (zmq_errno () == ENOTSUP);
#endif
}

void test__zmq_curve_public__valid__success ()
{
    // These are paired according to hintjens.com/blog:45
    static const char public_key[] = "Yne@$w-vo<fVvi]a<NY6T1ed:M$fCG*[IaLV{hID";
    static const char secret_key[] = "D:)Q[IlAW!ahhC2ac:9*A}h:p?([4%wOTJ%JR%cs";
    
    errno = 0;
    char out_public[41] = { 0 };
    
    const int rc = zmq_curve_public (out_public, secret_key);
    
#if defined (ZMQ_HAVE_CURVE)
    assert (rc == 0);
    assert (zmq_errno () == 0);
    assert (streq (out_public, public_key));
#else
    assert (rc == -1);
    assert (zmq_errno () == ENOTSUP);
    (void) public_key;
#endif
}

// The key length must be evenly divisible by 5 or must fail with EINVAL.
void test__zmq_curve_public__invalid__failure (const char *secret)
{
    errno = 0;
    char out_public[41] = { 0 };
    
    const int rc = zmq_curve_public(out_public, secret);
    
#if defined (ZMQ_HAVE_CURVE)
    assert (rc == -1);
    assert (zmq_errno () == EINVAL);
    assert (streq (out_public, ""));
#else
    assert (rc == -1);
    assert (zmq_errno () == ENOTSUP);
#endif
}

int test_sodium (void)
{
    NSLog(@"- test_sodium");
    test__zmq_curve_keypair__always__success ();
    
    test__zmq_curve_public__valid__success ();
    test__zmq_curve_public__invalid__failure ("42");
    test__zmq_curve_public__invalid__failure ("0123456789012345678901234567890123456789.");
    
    return 0;
}

const char *bind_address9 = 0;
const char *connect_address9 = 0;

void test_round_robin_out (void *ctx)
{
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    
    int rc = zmq_bind (dealer, bind_address9);
    assert (rc == 0);
    
    const size_t services = 5;
    void *rep [services];
    for (size_t peer = 0; peer < services; ++peer) {
        rep [peer] = zmq_socket (ctx, ZMQ_REP);
        assert (rep [peer]);
        
        int timeout = 250;
        rc = zmq_setsockopt (rep [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (rep [peer], connect_address9);
        assert (rc == 0);
    }
    
    // Wait for connections.
    msleep (SETTLE_TIME);
    
    // Send all requests
    for (size_t i = 0; i < services; ++i)
        s_send_seq (dealer, 0, "ABC", SEQ_END);
    
    // Expect every REP got one message
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    for (size_t peer = 0; peer < services; ++peer)
        s_recv_seq (rep [peer], "ABC", SEQ_END);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (dealer);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (rep [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_fair_queue_in (void *ctx)
{
    void *receiver = zmq_socket (ctx, ZMQ_DEALER);
    assert (receiver);
    
    int timeout = 250;
    int rc = zmq_setsockopt (receiver, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (receiver, bind_address9);
    assert (rc == 0);
    
    const size_t services = 5;
    void *senders [services];
    for (size_t peer = 0; peer < services; ++peer) {
        senders [peer] = zmq_socket (ctx, ZMQ_DEALER);
        assert (senders [peer]);
        
        rc = zmq_setsockopt (senders [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (senders [peer], connect_address9);
        assert (rc == 0);
    }
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    
    s_send_seq (senders [0], "A", SEQ_END);
    s_recv_seq (receiver, "A", SEQ_END);
    
    s_send_seq (senders [0], "A", SEQ_END);
    s_recv_seq (receiver, "A", SEQ_END);
    
    // send our requests
    for (size_t peer = 0; peer < services; ++peer)
        s_send_seq (senders [peer], "B", SEQ_END);
    
    // Wait for data.
    msleep (SETTLE_TIME);
    
    // handle the requests
    for (size_t peer = 0; peer < services; ++peer)
        s_recv_seq (receiver, "B", SEQ_END);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (receiver);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (senders [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_destroy_queue_on_disconnect (void *ctx)
{
    void *A = zmq_socket (ctx, ZMQ_DEALER);
    assert (A);
    
    int rc = zmq_bind (A, bind_address9);
    assert (rc == 0);
    
    void *B = zmq_socket (ctx, ZMQ_DEALER);
    assert (B);
    
    rc = zmq_connect (B, connect_address9);
    assert (rc == 0);
    
    // Send a message in both directions
    s_send_seq (A, "ABC", SEQ_END);
    s_send_seq (B, "DEF", SEQ_END);
    
    rc = zmq_disconnect (B, connect_address9);
    assert (rc == 0);
    
    // Disconnect may take time and need command processing.
    zmq_pollitem_t poller [2] = { { A, 0, 0, 0 }, { B, 0, 0, 0 } };
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    
    // No messages should be available, sending should fail.
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    rc = zmq_send (A, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_recv (&msg, A, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    // After a reconnect of B, the messages should still be gone
    rc = zmq_connect (B, connect_address9);
    assert (rc == 0);
    
    rc = zmq_msg_recv (&msg, A, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_recv (&msg, B, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (A);
    close_zero_linger (B);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_block_on_send_no_peers (void *ctx)
{
    void *sc = zmq_socket (ctx, ZMQ_DEALER);
    assert (sc);
    
    int timeout = 250;
    int rc = zmq_setsockopt (sc, ZMQ_SNDTIMEO, &timeout, sizeof (timeout));
    assert (rc == 0);
    
    rc = zmq_send (sc, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_send (sc, 0, 0, 0);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_close (sc);
    assert (rc == 0);
}

int test_spec_dealer (void)
{
    NSLog(@"- test_spec_dealer");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    const char *binds [] = { "inproc://a", "tcp://127.0.0.1:5555" };
    const char *connects [] = { "inproc://a", "tcp://localhost:5555" };
    
    for (int transports = 0; transports < 2; ++transports) {
        bind_address9 = binds [transports];
        connect_address9 = connects [transports];
        
        // SHALL route outgoing messages to available peers using a round-robin
        // strategy.
        test_round_robin_out (ctx);
        
        // SHALL receive incoming messages from its peers using a fair-queuing
        // strategy.
        test_fair_queue_in (ctx);
        
        // SHALL block on sending, or return a suitable error, when it has no connected peers.
        test_block_on_send_no_peers (ctx);
        
        // SHALL create a double queue when a peer connects to it. If this peer
        // disconnects, the DEALER socket SHALL destroy its double queue and SHALL
        // discard any messages it contains.
        // *** Test disabled until libzmq does this properly ***
        // test_destroy_queue_on_disconnect (ctx);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


const char *bind_address10 = 0;
const char *connect_address10 = 0;

void test_push_round_robin_out (void *ctx)
{
    void *push = zmq_socket (ctx, ZMQ_PUSH);
    assert (push);
    
    int rc = zmq_bind (push, bind_address10);
    assert (rc == 0);
    
    const size_t services = 5;
    void *pulls [services];
    for (size_t peer = 0; peer < services; ++peer) {
        pulls [peer] = zmq_socket (ctx, ZMQ_PULL);
        assert (pulls [peer]);
        
        int timeout = 250;
        rc = zmq_setsockopt (pulls [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (pulls [peer], connect_address10);
        assert (rc == 0);
    }
    
    // Wait for connections.
    msleep (SETTLE_TIME);
    
    // Send 2N messages
    for (size_t peer = 0; peer < services; ++peer)
        s_send_seq (push, "ABC", SEQ_END);
    for (size_t peer = 0; peer < services; ++peer)
        s_send_seq (push, "DEF", SEQ_END);
    
    // Expect every PULL got one of each
    for (size_t peer = 0; peer < services; ++peer) {
        s_recv_seq (pulls [peer], "ABC", SEQ_END);
        s_recv_seq (pulls [peer], "DEF", SEQ_END);
    }
    
    close_zero_linger (push);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (pulls [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_pull_fair_queue_in (void *ctx)
{
    void *pull = zmq_socket (ctx, ZMQ_PULL);
    assert (pull);
    
    int rc = zmq_bind (pull, bind_address10);
    assert (rc == 0);
    
    const size_t services = 5;
    void *pushs [services];
    for (size_t peer = 0; peer < services; ++peer)
    {
        pushs [peer] = zmq_socket (ctx, ZMQ_PUSH);
        assert (pushs [peer]);
        
        rc = zmq_connect (pushs [peer], connect_address10);
        assert (rc == 0);
    }
    
    // Wait for connections.
    msleep (SETTLE_TIME);
    
    int first_half = 0;
    int second_half = 0;
    
    // Send 2N messages
    for (size_t peer = 0; peer < services; ++peer) {
        char *str = strdup("A");
        
        str [0] += peer;
        s_send_seq (pushs [peer], str, SEQ_END);
        first_half += str [0];
        
        str [0] += services;
        s_send_seq (pushs [peer], str, SEQ_END);
        second_half += str [0];
        
        free (str);
    }
    
    // Wait for data.
    msleep (SETTLE_TIME);
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    
    // Expect to pull one from each first
    for (size_t peer = 0; peer < services; ++peer) {
        rc = zmq_msg_recv (&msg, pull, 0);
        assert (rc == 2);
        const char *str = (const char *)zmq_msg_data (&msg);
        first_half -= str [0];
    }
    assert (first_half == 0);
    
    // And then get the second batch
    for (size_t peer = 0; peer < services; ++peer) {
        rc = zmq_msg_recv (&msg, pull, 0);
        assert (rc == 2);
        const char *str = (const char *)zmq_msg_data (&msg);
        second_half -= str [0];
    }
    assert (second_half == 0);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (pull);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (pushs [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_push_block_on_send_no_peers (void *ctx)
{
    void *sc = zmq_socket (ctx, ZMQ_PUSH);
    assert (sc);
    
    int timeout = 250;
    int rc = zmq_setsockopt (sc, ZMQ_SNDTIMEO, &timeout, sizeof (timeout));
    assert (rc == 0);
    
    rc = zmq_send (sc, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_send (sc, 0, 0, 0);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_close (sc);
    assert (rc == 0);
}

void test_destroy_queue_on_disconnect10 (void *ctx)
{
    void *A = zmq_socket (ctx, ZMQ_PUSH);
    assert (A);
    
    int hwm = 1;
    int rc = zmq_setsockopt (A, ZMQ_SNDHWM, &hwm, sizeof (hwm));
    assert (rc == 0);
    
    rc = zmq_bind (A, bind_address10);
    assert (rc == 0);
    
    void *B = zmq_socket (ctx, ZMQ_PULL);
    assert (B);
    
    rc = zmq_setsockopt (B, ZMQ_RCVHWM, &hwm, sizeof (hwm));
    assert (rc == 0);
    
    rc = zmq_connect (B, connect_address10);
    assert (rc == 0);
    
    // Send two messages, one should be stuck in A's outgoing queue, the other
    // arrives at B.
    s_send_seq (A, "ABC", SEQ_END);
    s_send_seq (A, "DEF", SEQ_END);
    
    // Both queues should now be full, indicated by A blocking on send.
    rc = zmq_send (A, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_disconnect (B, connect_address10);
    assert (rc == 0);
    
    // Disconnect may take time and need command processing.
    zmq_pollitem_t poller [2] = { { A, 0, 0, 0 }, { B, 0, 0, 0 } };
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    
    // Can't receive old data on B.
    rc = zmq_msg_recv (&msg, B, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    // Sending fails.
    rc = zmq_send (A, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    // Reconnect B
    rc = zmq_connect (B, connect_address10);
    assert (rc == 0);
    
    // Still can't receive old data on B.
    rc = zmq_msg_recv (&msg, B, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    // two messages should be sendable before the queues are filled up.
    s_send_seq (A, "ABC", SEQ_END);
    s_send_seq (A, "DEF", SEQ_END);
    
    rc = zmq_send (A, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (A);
    close_zero_linger (B);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

int test_spec_pushpull (void)
{
    NSLog(@"- test_spec_pushpull");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    const char *binds [] = { "inproc://a", "tcp://127.0.0.1:5555" };
    const char *connects [] = { "inproc://a", "tcp://localhost:5555" };
    
    for (int transport = 0; transport < 2; ++transport) {
        bind_address10 = binds [transport];
        connect_address10 = connects [transport];
        
        // PUSH: SHALL route outgoing messages to connected peers using a
        // round-robin strategy.
        test_push_round_robin_out (ctx);
        
        // PULL: SHALL receive incoming messages from its peers using a fair-queuing
        // strategy.
        test_pull_fair_queue_in (ctx);
        
        // PUSH: SHALL block on sending, or return a suitable error, when it has no
        // available peers.
        test_push_block_on_send_no_peers (ctx);
        
        // PUSH and PULL: SHALL create this queue when a peer connects to it. If
        // this peer disconnects, the socket SHALL destroy its queue and SHALL
        // discard any messages it contains.
        // *** Test disabled until libzmq does this properly ***
        // test_destroy_queue_on_disconnect10 (ctx);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


const char *bind_address12 = 0;
const char *connect_address12 = 0;

void test_fair_queue_in12 (void *ctx)
{
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    
    int timeout = 250;
    int rc = zmq_setsockopt (rep, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (rep, bind_address12);
    assert (rc == 0);
    
    const size_t services = 5;
    void *reqs [services];
    for (size_t peer = 0; peer < services; ++peer) {
        reqs [peer] = zmq_socket (ctx, ZMQ_REQ);
        assert (reqs [peer]);
        
        rc = zmq_setsockopt (reqs [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (reqs [peer], connect_address12);
        assert (rc == 0);
    }
    
    msleep (SETTLE_TIME);
    
    s_send_seq (reqs [0], "A", SEQ_END);
    s_recv_seq (rep, "A", SEQ_END);
    s_send_seq (rep, "A", SEQ_END);
    s_recv_seq (reqs [0], "A", SEQ_END);
    
    s_send_seq (reqs [0], "A", SEQ_END);
    s_recv_seq (rep, "A", SEQ_END);
    s_send_seq (rep, "A", SEQ_END);
    s_recv_seq (reqs [0], "A", SEQ_END);
    
    // TODO: following test fails randomly on some boxes
#ifdef SOMEONE_FIXES_THIS
    // send N requests
    for (size_t peer = 0; peer < services; ++peer) {
        char * str = strdup("A");
        str [0] += peer;
        s_send_seq (reqs [peer], str, SEQ_END);
        free (str);
    }
    
    // handle N requests
    for (size_t peer = 0; peer < services; ++peer) {
        char * str = strdup("A");
        str [0] += peer;
        //  Test fails here
        s_recv_seq (rep, str, SEQ_END);
        s_send_seq (rep, str, SEQ_END);
        s_recv_seq (reqs [peer], str, SEQ_END);
        free (str);
    }
#endif
    close_zero_linger (rep);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (reqs [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_envelope12 (void *ctx)
{
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    
    int rc = zmq_bind (rep, bind_address12);
    assert (rc == 0);
    
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    
    rc = zmq_connect (dealer, connect_address12);
    assert (rc == 0);
    
    // minimal envelope
    s_send_seq (dealer, 0, "A", SEQ_END);
    s_recv_seq (rep, "A", SEQ_END);
    s_send_seq (rep, "A", SEQ_END);
    s_recv_seq (dealer, 0, "A", SEQ_END);
    
    // big envelope
    s_send_seq (dealer, "X", "Y", 0, "A", SEQ_END);
    s_recv_seq (rep, "A", SEQ_END);
    s_send_seq (rep, "A", SEQ_END);
    s_recv_seq (dealer, "X", "Y", 0, "A", SEQ_END);
    
    close_zero_linger (rep);
    close_zero_linger (dealer);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

int test_spec_rep (void)
{
    NSLog(@"- test_spec_rep");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    const char *binds [] = { "inproc://a", "tcp://127.0.0.1:5555" };
    const char *connects [] = { "inproc://a", "tcp://localhost:5555" };
    
    for (int transport = 0; transport < 2; ++transport) {
        bind_address12 = binds [transport];
        connect_address12 = connects [transport];
        
        // SHALL receive incoming messages from its peers using a fair-queuing
        // strategy.
        test_fair_queue_in12 (ctx);
        
        // For an incoming message:
        // SHALL remove and store the address envelope, including the delimiter.
        // SHALL pass the remaining data frames to its calling application.
        // SHALL wait for a single reply message from its calling application.
        // SHALL prepend the address envelope and delimiter.
        // SHALL deliver this message back to the originating peer.
        test_envelope12 (ctx);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


const char *bind_address13 = 0;
const char *connect_address13 = 0;

void test_round_robin_out13 (void *ctx)
{
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    int rc = zmq_bind (req, bind_address13);
    assert (rc == 0);
    
    const size_t services = 5;
    void *rep [services];
    for (size_t peer = 0; peer < services; peer++) {
        rep [peer] = zmq_socket (ctx, ZMQ_REP);
        assert (rep [peer]);
        
        int timeout = 250;
        rc = zmq_setsockopt (rep [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_connect (rep [peer], connect_address13);
        assert (rc == 0);
    }
    //  We have to give the connects time to finish otherwise the requests
    //  will not properly round-robin. We could alternatively connect the
    //  REQ sockets to the REP sockets.
    msleep (SETTLE_TIME);
    
    // Send our peer-replies, and expect every REP it used once in order
    for (size_t peer = 0; peer < services; peer++) {
        s_send_seq (req, "ABC", SEQ_END);
        s_recv_seq (rep [peer], "ABC", SEQ_END);
        s_send_seq (rep [peer], "DEF", SEQ_END);
        s_recv_seq (req, "DEF", SEQ_END);
    }
    
    close_zero_linger (req);
    for (size_t peer = 0; peer < services; peer++)
        close_zero_linger (rep [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_req_only_listens_to_current_peer (void *ctx)
{
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    int rc = zmq_setsockopt(req, ZMQ_IDENTITY, "A", 2);
    assert (rc == 0);
    
    rc = zmq_bind (req, bind_address13);
    assert (rc == 0);
    
    const size_t services = 3;
    void *router [services];
    
    for (size_t i = 0; i < services; ++i) {
        router [i] = zmq_socket (ctx, ZMQ_ROUTER);
        assert (router [i]);
        
        int timeout = 250;
        rc = zmq_setsockopt (router [i], ZMQ_RCVTIMEO, &timeout, sizeof (timeout));
        assert (rc == 0);
        
        int enabled = 1;
        rc = zmq_setsockopt (router [i], ZMQ_ROUTER_MANDATORY, &enabled, sizeof (enabled));
        assert (rc == 0);
        
        rc = zmq_connect (router [i], connect_address13);
        assert (rc == 0);
    }
    
    // Wait for connects to finish.
    msleep (SETTLE_TIME);
    
    for (size_t i = 0; i < services; ++i) {
        // There still is a race condition when a stale peer's message
        // arrives at the REQ just after a request was sent to that peer.
        // To avoid that happening in the test, sleep for a bit.
        rc = zmq_poll (0, 0, 10);
        assert (rc == 0);
        
        s_send_seq (req, "ABC", SEQ_END);
        
        // Receive on router i
        s_recv_seq (router [i], "A", 0, "ABC", SEQ_END);
        
        // Send back replies on all routers
        for (size_t j = 0; j < services; ++j) {
            const char *replies [] = { "WRONG", "GOOD" };
            const char *reply = replies [i == j ? 1 : 0];
            s_send_seq (router [j], "A", 0, reply, SEQ_END);
        }
        
        // Receive only the good reply
        s_recv_seq (req, "GOOD", SEQ_END);
    }
    
    close_zero_linger (req);
    for (size_t i = 0; i < services; ++i)
        close_zero_linger (router [i]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_req_message_format13 (void *ctx)
{
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    void *router = zmq_socket (ctx, ZMQ_ROUTER);
    assert (router);
    
    int rc = zmq_bind (req, bind_address13);
    assert (rc == 0);
    
    rc = zmq_connect (router, connect_address13);
    assert (rc == 0);
    
    // Send a multi-part request.
    s_send_seq (req, "ABC", "DEF", SEQ_END);
    
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    // Receive peer identity
    rc = zmq_msg_recv (&msg, router, 0);
    assert (rc != -1);
    assert (zmq_msg_size (&msg) > 0);
    zmq_msg_t peer_id_msg;
    zmq_msg_init (&peer_id_msg);
    zmq_msg_copy (&peer_id_msg, &msg);
    
    int more = 0;
    size_t more_size = sizeof (more);
    rc = zmq_getsockopt (router, ZMQ_RCVMORE, &more, &more_size);
    assert (rc == 0);
    assert (more);
    
    // Receive the rest.
    s_recv_seq (router, 0, "ABC", "DEF", SEQ_END);
    
    // Send back a single-part reply.
    rc = zmq_msg_send (&peer_id_msg, router, ZMQ_SNDMORE);
    assert (rc != -1);
    s_send_seq (router, 0, "GHI", SEQ_END);
    
    // Receive reply.
    s_recv_seq (req, "GHI", SEQ_END);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    rc = zmq_msg_close (&peer_id_msg);
    assert (rc == 0);
    
    close_zero_linger (req);
    close_zero_linger (router);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_block_on_send_no_peers13 (void *ctx)
{
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    int timeout = 250;
    int rc = zmq_setsockopt (sc, ZMQ_SNDTIMEO, &timeout, sizeof (timeout));
    assert (rc == 0);
    
    rc = zmq_send (sc, 0, 0, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_send (sc, 0, 0, 0);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_close (sc);
    assert (rc == 0);
}

int test_spec_req (void)
{
    NSLog(@"- test_spec_req");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    const char *binds [] = { "inproc://a", "tcp://127.0.0.1:5555" };
    const char *connects [] = { "inproc://a", "tcp://localhost:5555" };
    
    for (int transport = 0; transport < 2; transport++) {
        bind_address13 = binds [transport];
        connect_address13 = connects [transport];
        
        // SHALL route outgoing messages to connected peers using a round-robin
        // strategy.
        test_round_robin_out13 (ctx);
        
        // The request and reply messages SHALL have this format on the wire:
        // * A delimiter, consisting of an empty frame, added by the REQ socket.
        // * One or more data frames, comprising the message visible to the
        //   application.
        test_req_message_format13 (ctx);
        
        // SHALL block on sending, or return a suitable error, when it has no
        // connected peers.
        test_block_on_send_no_peers13 (ctx);
        
        // SHALL accept an incoming message only from the last peer that it sent a
        // request to.
        // SHALL discard silently any messages received from other peers.
        // PH: this test is still failing; disabled for now to allow build to
        // complete.
        // test_req_only_listens_to_current_peer (ctx);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

const char *bind_address14 = 0;
const char *connect_address14 = 0;

void test_fair_queue_in14 (void *ctx)
{
    void *receiver = zmq_socket (ctx, ZMQ_ROUTER);
    assert (receiver);
    
    int timeout = 250;
    int rc = zmq_setsockopt (receiver, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (receiver, bind_address14);
    assert (rc == 0);
    
    const size_t services = 5;
    void *senders [services];
    for (size_t peer = 0; peer < services; ++peer) {
        senders [peer] = zmq_socket (ctx, ZMQ_DEALER);
        assert (senders [peer]);
        
        rc = zmq_setsockopt (senders [peer], ZMQ_RCVTIMEO, &timeout, sizeof (int));
        assert (rc == 0);
        
        char *str = strdup("A");
        str [0] += peer;
        rc = zmq_setsockopt (senders [peer], ZMQ_IDENTITY, str, 2);
        assert (rc == 0);
        free (str);
        
        rc = zmq_connect (senders [peer], connect_address14);
        assert (rc == 0);
    }
    
    msleep (SETTLE_TIME);
    
    zmq_msg_t msg;
    rc = zmq_msg_init (&msg);
    assert (rc == 0);
    
    s_send_seq (senders [0], "M", SEQ_END);
    s_recv_seq (receiver, "A", "M", SEQ_END);
    
    s_send_seq (senders [0], "M", SEQ_END);
    s_recv_seq (receiver, "A", "M", SEQ_END);
    
    int sum = 0;
    
    // send N requests
    for (size_t peer = 0; peer < services; ++peer) {
        s_send_seq (senders [peer], "M", SEQ_END);
        sum += 'A' + peer;
    }
    
    assert (sum == services * 'A' + services * (services - 1) / 2);
    
    // handle N requests
    for (size_t peer = 0; peer < services; ++peer) {
        rc = zmq_msg_recv (&msg, receiver, 0);
        assert (rc == 2);
        const char *id = (const char *)zmq_msg_data (&msg);
        sum -= id [0];
        
        s_recv_seq (receiver, "M", SEQ_END);
    }
    
    assert (sum == 0);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (receiver);
    
    for (size_t peer = 0; peer < services; ++peer)
        close_zero_linger (senders [peer]);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}

void test_destroy_queue_on_disconnect14 (void *ctx)
{
    void *A = zmq_socket (ctx, ZMQ_ROUTER);
    assert (A);
    
    int enabled = 1;
    int rc = zmq_setsockopt (A, ZMQ_ROUTER_MANDATORY, &enabled, sizeof (enabled));
    assert (rc == 0);
    
    rc = zmq_bind (A, bind_address14);
    assert (rc == 0);
    
    void *B = zmq_socket (ctx, ZMQ_DEALER);
    assert (B);
    
    rc = zmq_setsockopt (B, ZMQ_IDENTITY, "B", 2);
    assert (rc == 0);
    
    rc = zmq_connect (B, connect_address14);
    assert (rc == 0);
    
    // Wait for connection.
    msleep (SETTLE_TIME);
    
    // Send a message in both directions
    s_send_seq (A, "B", "ABC", SEQ_END);
    s_send_seq (B, "DEF", SEQ_END);
    
    rc = zmq_disconnect (B, connect_address14);
    assert (rc == 0);
    
    // Disconnect may take time and need command processing.
    zmq_pollitem_t poller [2] = { { A, 0, 0, 0 }, { B, 0, 0, 0 } };
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    rc = zmq_poll (poller, 2, 100);
    assert (rc == 0);
    
    // No messages should be available, sending should fail.
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    
    rc = zmq_send (A, "B", 2, ZMQ_SNDMORE | ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EHOSTUNREACH);
    
    rc = zmq_msg_recv (&msg, A, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    // After a reconnect of B, the messages should still be gone
    rc = zmq_connect (B, connect_address14);
    assert (rc == 0);
    
    rc = zmq_msg_recv (&msg, A, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_recv (&msg, B, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (errno == EAGAIN);
    
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    close_zero_linger (A);
    close_zero_linger (B);
    
    // Wait for disconnects.
    msleep (SETTLE_TIME);
}


int test_spec_router (void)
{
    NSLog(@"- test_spec_router");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    const char *binds [] = { "inproc://a", "tcp://127.0.0.1:5555" };
    const char *connects [] = { "inproc://a", "tcp://localhost:5555" };
    
    for (int transport = 0; transport < 2; ++transport) {
        bind_address14 = binds [transport];
        connect_address14 = connects [transport];
        
        // SHALL receive incoming messages from its peers using a fair-queuing
        // strategy.
        test_fair_queue_in14 (ctx);
        
        // SHALL create a double queue when a peer connects to it. If this peer
        // disconnects, the ROUTER socket SHALL destroy its double queue and SHALL
        // discard any messages it contains.
        // *** Test disabled until libzmq does this properly ***
        // test_destroy_queue_on_disconnect14 (ctx);
    }
    
    int rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


#define MSG_SIZE 20

int test_srcfd (void)
{
    NSLog(@"- test_srcfd");
    int rc;
    
    setup_test_environment();
    //  Create the infrastructure
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *rep = zmq_socket (ctx, ZMQ_REP);
    assert (rep);
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    
    rc = zmq_bind(rep, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    rc = zmq_connect(req, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    char tmp[MSG_SIZE];
    memset (tmp, 0, MSG_SIZE);
    zmq_send(req, tmp, MSG_SIZE, 0);
    
    zmq_msg_t msg;
    rc = zmq_msg_init(&msg);
    assert (rc == 0);
    
    zmq_recvmsg(rep, &msg, 0);
    assert(zmq_msg_size(&msg) == MSG_SIZE);
    
    // get the messages source file descriptor
    int srcFd = zmq_msg_get(&msg, ZMQ_SRCFD);
    assert(srcFd >= 0);
    
    rc = zmq_msg_close(&msg);
    assert (rc == 0);
    
    // get the remote endpoint
    struct sockaddr_storage ss;
#ifdef ZMQ_HAVE_HPUX
    int addrlen = sizeof ss;
#else
    socklen_t addrlen = sizeof ss;
#endif
    rc = getpeername (srcFd, (struct sockaddr*) &ss, &addrlen);
    assert (rc == 0);
    
    char host [NI_MAXHOST];
    rc = getnameinfo ((struct sockaddr*) &ss, addrlen, host, sizeof host,
                      NULL, 0, NI_NUMERICHOST);
    assert (rc == 0);
    
    // assert it is localhost which connected
    assert (strcmp(host, "127.0.0.1") == 0);
    
    rc = zmq_close (rep);
    assert (rc == 0);
    rc = zmq_close (req);
    assert (rc == 0);
    
    // sleep a bit for the socket to be freed
    msleep (SETTLE_TIME);
    
    // getting name from closed socket will fail
    rc = getpeername (srcFd, (struct sockaddr*) &ss, &addrlen);
#ifdef ZMQ_HAVE_WINDOWS
    assert (rc == SOCKET_ERROR);
    assert (WSAGetLastError() == WSAENOTSOCK);
#else
    assert (rc == -1);
    assert (errno == EBADF);
#endif
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


//  ZMTP protocol greeting structure

typedef unsigned char byte;
typedef struct {
    byte signature [10];    //  0xFF 8*0x00 0x7F
    byte version [2];       //  0x03 0x00 for ZMTP/3.0
    byte mechanism [20];    //  "NULL"
    byte as_server;
    byte filler [31];
} zmtp_greeting_t;

#define ZMTP_DEALER  5      //  Socket type constants

//  This is a greeting matching what 0MQ will send us; note the
//  8-byte size is set to 1 for backwards compatibility

static zmtp_greeting_t
greeting = { { 0xFF, 0, 0, 0, 0, 0, 0, 0, 1, 0x7F },
    { 3, 0 },
    { 'N', 'U', 'L', 'L'},
    0,
    { 0 }
};

static void
test_stream_to_dealer (void)
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  We'll be using this socket in raw mode
    void *stream = zmq_socket (ctx, ZMQ_STREAM);
    assert (stream);
    
    int zero = 0;
    rc = zmq_setsockopt (stream, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    int enabled = 1;
    rc = zmq_setsockopt (stream, ZMQ_STREAM_NOTIFY, &enabled, sizeof (enabled));
    assert (rc == 0);
    rc = zmq_bind (stream, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    //  We'll be using this socket as the other peer
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    rc = zmq_setsockopt (dealer, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_connect (dealer, "tcp://localhost:5556");
    
    //  Send a message on the dealer socket
    rc = zmq_send (dealer, "Hello", 5, 0);
    assert (rc == 5);
    
    //  Connecting sends a zero message
    //  First frame is identity
    zmq_msg_t identity;
    rc = zmq_msg_init (&identity);
    assert (rc == 0);
    rc = zmq_msg_recv (&identity, stream, 0);
    assert (rc > 0);
    assert (zmq_msg_more (&identity));
    
    //  Verify the existence of Peer-Address metadata
    char const *peer_address = zmq_msg_gets (&identity, "Peer-Address");
    assert (peer_address != 0);
    assert (streq (peer_address, "127.0.0.1"));
    
    //  Second frame is zero
    byte buffer [255];
    rc = zmq_recv (stream, buffer, 255, 0);
    assert (rc == 0);
    
    //  Verify the existence of Peer-Address metadata
    peer_address = zmq_msg_gets (&identity, "Peer-Address");
    assert (peer_address != 0);
    assert (streq (peer_address, "127.0.0.1"));
    
    //  Real data follows
    //  First frame is identity
    rc = zmq_msg_recv (&identity, stream, 0);
    assert (rc > 0);
    assert (zmq_msg_more (&identity));
    
    //  Verify the existence of Peer-Address metadata
    peer_address = zmq_msg_gets (&identity, "Peer-Address");
    assert (peer_address != 0);
    assert (streq (peer_address, "127.0.0.1"));
    
    //  Second frame is greeting signature
    rc = zmq_recv (stream, buffer, 255, 0);
    assert (rc == 10);
    assert (memcmp (buffer, greeting.signature, 10) == 0);
    
    //  Send our own protocol greeting
    rc = zmq_msg_send (&identity, stream, ZMQ_SNDMORE);
    assert (rc > 0);
    rc = zmq_send (stream, &greeting, sizeof (greeting), 0);
    assert (rc == sizeof (greeting));
    
    //  Now we expect the data from the DEALER socket
    //  We want the rest of greeting along with the Ready command
    int bytes_read = 0;
    while (bytes_read < 97) {
        //  First frame is the identity of the connection (each time)
        rc = zmq_msg_recv (&identity, stream, 0);
        assert (rc > 0);
        assert (zmq_msg_more (&identity));
        //  Second frame contains the next chunk of data
        rc = zmq_recv (stream, buffer + bytes_read, 255 - bytes_read, 0);
        assert (rc >= 0);
        bytes_read += rc;
    }
    
    //  First two bytes are major and minor version numbers.
    assert (buffer [0] == 3);       //  ZMTP/3.0
    assert (buffer [1] == 0);
    
    //  Mechanism is "NULL"
    assert (memcmp (buffer + 2, "NULL\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", 20) == 0);
    assert (memcmp (buffer + 54, "\4\51\5READY", 8) == 0);
    assert (memcmp (buffer + 62, "\13Socket-Type\0\0\0\6DEALER", 22) == 0);
    assert (memcmp (buffer + 84, "\10Identity\0\0\0\0", 13) == 0);
    
    //  Announce we are ready
    memcpy (buffer, "\4\51\5READY", 8);
    memcpy (buffer + 8, "\13Socket-Type\0\0\0\6ROUTER", 22);
    memcpy (buffer + 30, "\10Identity\0\0\0\0", 13);
    
    //  Send Ready command
    rc = zmq_msg_send (&identity, stream, ZMQ_SNDMORE);
    assert (rc > 0);
    rc = zmq_send (stream, buffer, 43, 0);
    assert (rc == 43);
    
    //  Now we expect the data from the DEALER socket
    //  First frame is, again, the identity of the connection
    rc = zmq_msg_recv (&identity, stream, 0);
    assert (rc > 0);
    assert (zmq_msg_more (&identity));
    
    //  Third frame contains Hello message from DEALER
    rc = zmq_recv (stream, buffer, sizeof buffer, 0);
    assert (rc == 7);
    
    //  Then we have a 5-byte message "Hello"
    assert (buffer [0] == 0);       //  Flags = 0
    assert (buffer [1] == 5);       //  Size = 5
    assert (memcmp (buffer + 2, "Hello", 5) == 0);
    
    //  Send "World" back to DEALER
    rc = zmq_msg_send (&identity, stream, ZMQ_SNDMORE);
    assert (rc > 0);
    byte world [] = { 0, 5, 'W', 'o', 'r', 'l', 'd' };
    rc = zmq_send (stream, world, sizeof (world), 0);
    assert (rc == sizeof (world));
    
    //  Expect response on DEALER socket
    rc = zmq_recv (dealer, buffer, 255, 0);
    assert (rc == 5);
    assert (memcmp (buffer, "World", 5) == 0);
    
    //  Test large messages over STREAM socket
#define SIZE1 64000
    uint8_t msgout [SIZE1];
    memset (msgout, 0xAB, SIZE1);
    zmq_send (dealer, msgout, SIZE1, 0);
    
    uint8_t msgin [9 + SIZE1];
    memset (msgin, 0, 9 + SIZE1);
    bytes_read = 0;
    while (bytes_read < 9 + SIZE1) {
        //  Get identity frame
        rc = zmq_recv (stream, buffer, 256, 0);
        assert (rc > 0);
        //  Get next chunk
        rc = zmq_recv (stream, msgin + bytes_read, 9 + SIZE1 - bytes_read, 0);
        assert (rc > 0);
        bytes_read += rc;
    }
    int byte_nbr;
    for (byte_nbr = 0; byte_nbr < SIZE1; byte_nbr++) {
        if (msgin [9 + byte_nbr] != 0xAB)
            assert (false);
    }
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_close (stream);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}


static void
test_stream_to_stream (void)
{
    int rc;
    //  Set-up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *server = zmq_socket (ctx, ZMQ_STREAM);
    assert (server);
    int enabled = 1;
    rc = zmq_setsockopt (server, ZMQ_STREAM_NOTIFY, &enabled, sizeof (enabled));
    assert (rc == 0);
    rc = zmq_bind (server, "tcp://127.0.0.1:9070");
    assert (rc == 0);
    
    void *client = zmq_socket (ctx, ZMQ_STREAM);
    assert (client);
    rc = zmq_setsockopt (client, ZMQ_STREAM_NOTIFY, &enabled, sizeof (enabled));
    assert (rc == 0);
    rc = zmq_connect (client, "tcp://localhost:9070");
    assert (rc == 0);
    uint8_t id [256];
    size_t id_size = 256;
    uint8_t buffer [256];
    
    //  Connecting sends a zero message
    //  Server: First frame is identity, second frame is zero
    id_size = zmq_recv (server, id, 256, 0);
    assert (id_size > 0);
    rc = zmq_recv (server, buffer, 256, 0);
    assert (rc == 0);
    //  Client: First frame is identity, second frame is zero
    id_size = zmq_recv (client, id, 256, 0);
    assert (id_size > 0);
    rc = zmq_recv (client, buffer, 256, 0);
    assert (rc == 0);
    
    //  Sent HTTP request on client socket
    //  Get server identity
    rc = zmq_getsockopt (client, ZMQ_IDENTITY, id, &id_size);
    assert (rc == 0);
    //  First frame is server identity
    rc = zmq_send (client, id, id_size, ZMQ_SNDMORE);
    assert (rc == (int) id_size);
    //  Second frame is HTTP GET request
    rc = zmq_send (client, "GET /\n\n", 7, 0);
    assert (rc == 7);
    
    //  Get HTTP request; ID frame and then request
    id_size = zmq_recv (server, id, 256, 0);
    assert (id_size > 0);
    rc = zmq_recv (server, buffer, 256, 0);
    assert (rc != -1);
    assert (memcmp (buffer, "GET /\n\n", 7) == 0);
    
    //  Send reply back to client
    char http_response [] =
    "HTTP/1.0 200 OK\r\n"
    "Content-Type: text/plain\r\n"
    "\r\n"
    "Hello, World!";
    rc = zmq_send (server, id, id_size, ZMQ_SNDMORE);
    assert (rc != -1);
    rc = zmq_send (server, http_response, sizeof (http_response), ZMQ_SNDMORE);
    assert (rc != -1);
    
    //  Send zero to close connection to client
    rc = zmq_send (server, id, id_size, ZMQ_SNDMORE);
    assert (rc != -1);
    rc = zmq_send (server, NULL, 0, ZMQ_SNDMORE);
    assert (rc != -1);
    
    //  Get reply at client and check that it's complete
    id_size = zmq_recv (client, id, 256, 0);
    assert (id_size > 0);
    rc = zmq_recv (client, buffer, 256, 0);
    assert (rc == sizeof (http_response));
    assert (memcmp (buffer, http_response, sizeof (http_response)) == 0);
    
    // //  Get disconnection notification
    // FIXME: why does this block? Bug in STREAM disconnect notification?
    // id_size = zmq_recv (client, id, 256, 0);
    // assert (id_size > 0);
    // rc = zmq_recv (client, buffer, 256, 0);
    // assert (rc == 0);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_stream(void)
{
    NSLog(@"- test_stream");
    setup_test_environment();
    test_stream_to_dealer ();
    test_stream_to_stream ();
}

static const int SERVER15 = 0;
static const int CLIENT15 = 1;

struct test_message_t {
    int turn;
    const char * text;
};

// NOTE: messages are sent without null terminator.
const test_message_t dialog [] = {
    {CLIENT15, "i can haz cheez burger?"},
    {SERVER15, "y u no disonnect?"},
    {CLIENT15, ""},
};

const int steps15 = sizeof(dialog) / sizeof(dialog[0]);

bool has_more15 (void* socket)
{
    int more = 0;
    size_t more_size = sizeof(more);
    int rc = zmq_getsockopt (socket, ZMQ_RCVMORE, &more, &more_size);
    if (rc != 0)
        return false;
    return more != 0;
}

bool get_identity (void* socket, char* data, size_t* size2)
{
    int rc = zmq_getsockopt (socket, ZMQ_IDENTITY, data, size2);
    return rc == 0;
}

int test_stream_disconnect(void	)
{
    NSLog(@"- test_stream_disconnect");
    setup_test_environment();
    
    void *context = zmq_ctx_new ();
    void *sockets [2];
    int rc = 0;
    
    sockets [SERVER15] = zmq_socket (context, ZMQ_STREAM);
    int enabled = 1;
    rc = zmq_setsockopt (sockets [SERVER15], ZMQ_STREAM_NOTIFY, &enabled, sizeof (enabled));
    assert (rc == 0);
    rc = zmq_bind (sockets [SERVER15], "tcp://0.0.0.0:6666");
    assert (rc == 0);
    
    sockets [CLIENT15] = zmq_socket (context, ZMQ_STREAM);
    rc = zmq_setsockopt (sockets [CLIENT15], ZMQ_STREAM_NOTIFY, &enabled, sizeof (enabled));
    assert (rc == 0);
    rc = zmq_connect (sockets [CLIENT15], "tcp://localhost:6666");
    assert (rc == 0);
    
    // wait for connect notification
    // Server: Grab the 1st frame (peer identity).
    zmq_msg_t peer_frame;
    rc = zmq_msg_init (&peer_frame);
    assert (rc == 0);
    rc = zmq_msg_recv (&peer_frame, sockets [SERVER15], 0);
    assert (rc != -1);
    assert(zmq_msg_size (&peer_frame) > 0);
    assert (has_more15 (sockets [SERVER15]));
    rc = zmq_msg_close (&peer_frame);
    assert (rc == 0);
    
    // Server: Grab the 2nd frame (actual payload).
    zmq_msg_t data_frame;
    rc = zmq_msg_init (&data_frame);
    assert (rc == 0);
    rc = zmq_msg_recv (&data_frame, sockets [SERVER15], 0);
    assert (rc != -1);
    assert(zmq_msg_size (&data_frame) == 0);
    rc = zmq_msg_close (&data_frame);
    assert (rc == 0);
    
    // Client: Grab the 1st frame (peer identity).
    rc = zmq_msg_init (&peer_frame);
    assert (rc == 0);
    rc = zmq_msg_recv (&peer_frame, sockets [CLIENT15], 0);
    assert (rc != -1);
    assert(zmq_msg_size (&peer_frame) > 0);
    assert (has_more15 (sockets [CLIENT15]));
    rc = zmq_msg_close (&peer_frame);
    assert (rc == 0);
    
    // Client: Grab the 2nd frame (actual payload).
    rc = zmq_msg_init (&data_frame);
    assert (rc == 0);
    rc = zmq_msg_recv (&data_frame, sockets [CLIENT15], 0);
    assert (rc != -1);
    assert(zmq_msg_size (&data_frame) == 0);
    rc = zmq_msg_close (&data_frame);
    assert (rc == 0);
    
    // Send initial message.
    char blob_data [256];
    size_t blob_size = sizeof(blob_data);
    rc = zmq_getsockopt (sockets [CLIENT15], ZMQ_IDENTITY, blob_data, &blob_size);
    assert (rc != -1);
    assert(blob_size > 0);
    zmq_msg_t msg;
    rc = zmq_msg_init_size (&msg, blob_size);
    assert (rc == 0);
    memcpy (zmq_msg_data (&msg), blob_data, blob_size);
    rc = zmq_msg_send (&msg, sockets [dialog [0].turn], ZMQ_SNDMORE);
    assert (rc != -1);
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    rc = zmq_msg_init_size (&msg, strlen(dialog [0].text));
    assert (rc == 0);
    memcpy (zmq_msg_data (&msg), dialog [0].text, strlen(dialog [0].text));
    rc = zmq_msg_send (&msg, sockets [dialog [0].turn], ZMQ_SNDMORE);
    assert (rc != -1);
    rc = zmq_msg_close (&msg);
    assert (rc == 0);
    
    // TODO: make sure this loop doesn't loop forever if something is wrong
    //       with the test (or the implementation).
    
    int step = 0;
    while (step < steps15) {
        // Wait until something happens.
        zmq_pollitem_t items [] = {
            { sockets [SERVER15], 0, ZMQ_POLLIN, 0 },
            { sockets [CLIENT15], 0, ZMQ_POLLIN, 0 },
        };
        int rc = zmq_poll (items, 2, 100);
        assert (rc >= 0);
        
        // Check for data received by the server.
        if (items [SERVER15].revents & ZMQ_POLLIN) {
            assert (dialog [step].turn == CLIENT15);
            
            // Grab the 1st frame (peer identity).
            zmq_msg_t peer_frame;
            rc = zmq_msg_init (&peer_frame);
            assert (rc == 0);
            rc = zmq_msg_recv (&peer_frame, sockets [SERVER15], 0);
            assert (rc != -1);
            assert(zmq_msg_size (&peer_frame) > 0);
            assert (has_more15 (sockets [SERVER15]));
            
            // Grab the 2nd frame (actual payload).
            zmq_msg_t data_frame;
            rc = zmq_msg_init (&data_frame);
            assert (rc == 0);
            rc = zmq_msg_recv (&data_frame, sockets [SERVER15], 0);
            assert (rc != -1);
            
            // Make sure payload matches what we expect.
            const char * const data = (const char*)zmq_msg_data (&data_frame);
            const int size1 = zmq_msg_size (&data_frame);
            // 0-length frame is a disconnection notification.  The server
            // should receive it as the last step in the dialogue.
            if (size1 == 0) {
                ++step;
                assert (step == steps15);
            }
            else {
                assert ((size_t) size1 == strlen (dialog [step].text));
                int cmp = memcmp (dialog [step].text, data, size1);
                assert (cmp == 0);
                
                ++step;
                
                assert (step < steps15);
                
                // Prepare the response.
                rc = zmq_msg_close (&data_frame);
                assert (rc == 0);
                rc = zmq_msg_init_size (&data_frame,
                                        strlen (dialog [step].text));
                assert (rc == 0);
                memcpy (zmq_msg_data (&data_frame), dialog [step].text,
                        zmq_msg_size (&data_frame));
                
                // Send the response.
                rc = zmq_msg_send (&peer_frame, sockets [SERVER15], ZMQ_SNDMORE);
                assert (rc != -1);
                rc = zmq_msg_send (&data_frame, sockets [SERVER15], ZMQ_SNDMORE);
                assert (rc != -1);
            }
            
            // Release resources.
            rc = zmq_msg_close (&peer_frame);
            assert (rc == 0);
            rc = zmq_msg_close (&data_frame);
            assert (rc == 0);
        }
        
        // Check for data received by the client.
        if (items [CLIENT15].revents & ZMQ_POLLIN) {
            assert (dialog [step].turn == SERVER15);
            
            // Grab the 1st frame (peer identity).
            zmq_msg_t peer_frame;
            rc = zmq_msg_init (&peer_frame);
            assert (rc == 0);
            rc = zmq_msg_recv (&peer_frame, sockets [CLIENT15], 0);
            assert (rc != -1);
            assert(zmq_msg_size (&peer_frame) > 0);
            assert (has_more15 (sockets [CLIENT15]));
            
            // Grab the 2nd frame (actual payload).
            zmq_msg_t data_frame;
            rc = zmq_msg_init (&data_frame);
            assert (rc == 0);
            rc = zmq_msg_recv (&data_frame, sockets [CLIENT15], 0);
            assert (rc != -1);
            assert(zmq_msg_size (&data_frame) > 0);
            
            // Make sure payload matches what we expect.
            const char * const data = (const char*)zmq_msg_data (&data_frame);
            const int size1 = zmq_msg_size (&data_frame);
            assert ((size_t)size1 == strlen(dialog [step].text));
            int cmp = memcmp(dialog [step].text, data, size1);
            assert (cmp == 0);
            
            ++step;
            
            // Prepare the response (next line in the dialog).
            assert (step < steps15);
            rc = zmq_msg_close (&data_frame);
            assert (rc == 0);
            rc = zmq_msg_init_size (&data_frame, strlen (dialog [step].text));
            assert (rc == 0);
            memcpy (zmq_msg_data (&data_frame), dialog [step].text, zmq_msg_size (&data_frame));
            
            // Send the response.
            rc = zmq_msg_send (&peer_frame, sockets [CLIENT15], ZMQ_SNDMORE);
            assert (rc != -1);
            rc = zmq_msg_send (&data_frame, sockets [CLIENT15], ZMQ_SNDMORE);
            assert (rc != -1);
            
            // Release resources.
            rc = zmq_msg_close (&peer_frame);
            assert (rc == 0);
            rc = zmq_msg_close (&data_frame);
            assert (rc == 0);
        }
    }
    assert (step == steps15);
    rc = zmq_close (sockets [CLIENT15]);
    assert (rc == 0);
    rc = zmq_close (sockets [SERVER15]);
    assert (rc == 0);
    rc = zmq_ctx_term (context);
    assert (rc == 0);
    return 0;
}


void test_stream_empty (void)
{
    NSLog(@"- test_stream_empty");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *stream = zmq_socket (ctx, ZMQ_STREAM);
    assert (stream);
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    
    int rc = zmq_bind (stream, "tcp://127.0.0.1:5555");
    assert (rc >= 0);
    rc = zmq_connect (dealer, "tcp://127.0.0.1:5555");
    assert (rc >= 0);
    zmq_send (dealer, "", 0, 0);
    
    zmq_msg_t ident, empty;
    zmq_msg_init (&ident);
    rc = zmq_msg_recv (&ident, stream, 0);
    assert (rc >= 0);
    rc = zmq_msg_init_data (&empty, (void *) "", 0, NULL, NULL);
    assert (rc >= 0);
    
    rc = zmq_msg_send (&ident, stream, ZMQ_SNDMORE);
    assert (rc >= 0);
    rc = zmq_msg_close (&ident);
    assert (rc >= 0);
    
    rc = zmq_msg_send (&empty, stream, 0);
    assert (rc >= 0);
    
    //  This close used to fail with Bad Address
    rc = zmq_msg_close (&empty);
    assert (rc >= 0);
    
    close_zero_linger (dealer);
    close_zero_linger (stream);
    zmq_ctx_term (ctx);
}


void test_stream_exceeds_buffer()
{
    NSLog(@"- test_stream_exceeds_buffer");
    const int msgsize = 8193;
    char sndbuf[msgsize] = "\xde\xad\xbe\xef";
    unsigned char rcvbuf[msgsize];
    
    int server_sock = socket(AF_INET, SOCK_STREAM, 0);
    assert(server_sock!=-1);
    int enable = 1;
    int rc = setsockopt (server_sock, SOL_SOCKET, SO_REUSEADDR, (char *) &enable, sizeof(enable));
    assert(rc!=-1);
    
    struct sockaddr_in saddr;
    memset(&saddr, 0, sizeof(saddr));
    saddr.sin_family = AF_INET;
    saddr.sin_addr.s_addr = INADDR_ANY;
    saddr.sin_port = htons(12345);
    
    rc = bind(server_sock, (struct sockaddr *)&saddr, sizeof(saddr));
    assert(rc!=-1);
    rc = listen(server_sock, 1);
    assert(rc!=-1);
    
    void *zctx = zmq_ctx_new();
    assert(zctx);
    void *zsock = zmq_socket(zctx, ZMQ_STREAM);
    assert(zsock);
    rc = zmq_connect(zsock, "tcp://127.0.0.1:12345");
    assert(rc!=-1);
    
    int client_sock = accept(server_sock, NULL, NULL);
    assert(client_sock!=-1);
    
    rc = close(server_sock);
    assert(rc!=-1);
    
    rc = send(client_sock, sndbuf, msgsize, 0);
    assert(rc==msgsize);
    
    zmq_msg_t msg;
    zmq_msg_init(&msg);
    
    int rcvbytes = 0;
    while (rcvbytes==0) // skip connection notification, if any
    {
        rc = zmq_msg_recv(&msg, zsock, 0);  // peerid
        assert(rc!=-1);
        assert(zmq_msg_more(&msg));
        rcvbytes = zmq_msg_recv(&msg, zsock, 0);
        assert(rcvbytes!=-1);
        assert(!zmq_msg_more(&msg));
    }
    
    // for this test, we only collect the first chunk
    // since the corruption already occurs in the first chunk
    memcpy(rcvbuf, zmq_msg_data(&msg), zmq_msg_size(&msg));
    
    zmq_msg_close(&msg);
    zmq_close(zsock);
    close(client_sock);
    
    zmq_ctx_destroy(zctx);
    
    assert(rcvbytes >= 4);
    
    // notice that only the 1st byte gets corrupted
    assert(rcvbuf[3]==0xef);
    assert(rcvbuf[2]==0xbe);
    assert(rcvbuf[1]==0xad);
    assert(rcvbuf[0]==0xde);
    
    (void)(rc); // avoid -Wunused-but-set-variable warning in release build
}



//  Read one event off the monitor socket; return value and address
//  by reference, if not null, and event number by value. Returns -1
//  in case of error.

static int
get_monitor_event20 (void *monitor, int *value, char **address)
{
    //  First frame in message contains event number and value
    zmq_msg_t msg;
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (zmq_msg_more (&msg));
    
    uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
    uint16_t event = *(uint16_t *) (data);
    if (value)
        *value = *(uint32_t *) (data + 2);
    
    //  Second frame in message contains event address
    zmq_msg_init (&msg);
    if (zmq_msg_recv (&msg, monitor, 0) == -1)
        return -1;              //  Interruped, presumably
    assert (!zmq_msg_more (&msg));
    
    if (address) {
        uint8_t *data = (uint8_t *) zmq_msg_data (&msg);
        size_t size = zmq_msg_size (&msg);
        *address = (char *) malloc (size + 1);
        memcpy (*address, data, size);
        *address [size] = 0;
    }
    return event;
}

static void
test_stream_handshake_timeout_accept20 (void)
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  We use this socket in raw mode, to make a connection and send nothing
    void *stream = zmq_socket (ctx, ZMQ_STREAM);
    assert (stream);
    
    int zero = 0;
    rc = zmq_setsockopt (stream, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_connect (stream, "tcp://localhost:5557");
    assert (rc == 0);
    
    //  We'll be using this socket to test TCP stream handshake timeout
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    rc = zmq_setsockopt (dealer, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    int val, tenth = 1;
    size_t vsize = sizeof(val);
    
    // check for the expected default handshake timeout value - 30 sec
    rc = zmq_getsockopt (dealer, ZMQ_HANDSHAKE_IVL, &val, &vsize);
    assert (rc == 0);
    assert (vsize == sizeof(val));
    assert (val == 30000);
    // make handshake timeout faster - 1/10 sec
    rc = zmq_setsockopt (dealer, ZMQ_HANDSHAKE_IVL, &tenth, sizeof (tenth));
    assert (rc == 0);
    vsize = sizeof(val);
    // make sure zmq_setsockopt changed the value
    rc = zmq_getsockopt (dealer, ZMQ_HANDSHAKE_IVL, &val, &vsize);
    assert (rc == 0);
    assert (vsize == sizeof(val));
    assert (val == tenth);
    
    //  Create and connect a socket for collecting monitor events on dealer
    void *dealer_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (dealer_mon);
    
    rc = zmq_socket_monitor (dealer, "inproc://monitor-dealer",
                             ZMQ_EVENT_CONNECTED | ZMQ_EVENT_DISCONNECTED | ZMQ_EVENT_ACCEPTED);
    assert (rc == 0);
    
    //  Connect to the inproc endpoint so we'll get events
    rc = zmq_connect (dealer_mon, "inproc://monitor-dealer");
    assert (rc == 0);
    
    // bind dealer socket to accept connection from non-sending stream socket
    rc = zmq_bind (dealer, "tcp://127.0.0.1:5557");
    assert (rc == 0);
    
    // we should get ZMQ_EVENT_ACCEPTED and then ZMQ_EVENT_DISCONNECTED
    int event = get_monitor_event20 (dealer_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_ACCEPTED);
    event = get_monitor_event20 (dealer_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_DISCONNECTED);
    
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_close (dealer_mon);
    assert (rc == 0);
    
    rc = zmq_close (stream);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

static void
test_stream_handshake_timeout_connect20 (void)
{
    int rc;
    
    //  Set up our context and sockets
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  We use this socket in raw mode, to accept a connection and send nothing
    void *stream = zmq_socket (ctx, ZMQ_STREAM);
    assert (stream);
    
    int zero = 0;
    rc = zmq_setsockopt (stream, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    rc = zmq_bind (stream, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    //  We'll be using this socket to test TCP stream handshake timeout
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    assert (dealer);
    rc = zmq_setsockopt (dealer, ZMQ_LINGER, &zero, sizeof (zero));
    assert (rc == 0);
    int val, tenth = 1;
    size_t vsize = sizeof(val);
    
    // check for the expected default handshake timeout value - 30 sec
    rc = zmq_getsockopt (dealer, ZMQ_HANDSHAKE_IVL, &val, &vsize);
    assert (rc == 0);
    assert (vsize == sizeof(val));
    assert (val == 30000);
    // make handshake timeout faster - 1/10 sec
    rc = zmq_setsockopt (dealer, ZMQ_HANDSHAKE_IVL, &tenth, sizeof (tenth));
    assert (rc == 0);
    vsize = sizeof(val);
    // make sure zmq_setsockopt changed the value
    rc = zmq_getsockopt (dealer, ZMQ_HANDSHAKE_IVL, &val, &vsize);
    assert (rc == 0);
    assert (vsize == sizeof(val));
    assert (val == tenth);
    
    //  Create and connect a socket for collecting monitor events on dealer
    void *dealer_mon = zmq_socket (ctx, ZMQ_PAIR);
    assert (dealer_mon);
    
    rc = zmq_socket_monitor (dealer, "inproc://monitor-dealer",
                             ZMQ_EVENT_CONNECTED | ZMQ_EVENT_DISCONNECTED | ZMQ_EVENT_ACCEPTED);
    assert (rc == 0);
    
    //  Connect to the inproc endpoint so we'll get events
    rc = zmq_connect (dealer_mon, "inproc://monitor-dealer");
    assert (rc == 0);
    
    // connect dealer socket to non-sending stream socket
    rc = zmq_connect (dealer, "tcp://localhost:5556");
    assert (rc == 0);
    
    // we should get ZMQ_EVENT_CONNECTED and then ZMQ_EVENT_DISCONNECTED
    int event = get_monitor_event20 (dealer_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_CONNECTED);
    event = get_monitor_event20 (dealer_mon, NULL, NULL);
    assert (event == ZMQ_EVENT_DISCONNECTED);
    
    rc = zmq_close (dealer);
    assert (rc == 0);
    
    rc = zmq_close (dealer_mon);
    assert (rc == 0);
    
    rc = zmq_close (stream);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
}

void test_stream_timeout (void)
{
    NSLog(@"- test_stream_timeout");
    setup_test_environment();
    test_stream_handshake_timeout_connect20 ();
    test_stream_handshake_timeout_accept20 ();
}


int test_sub_forward (void)
{
    NSLog(@"- test_sub_forward");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  First, create an intermediate device
    void *xpub = zmq_socket (ctx, ZMQ_XPUB);
    assert (xpub);
    int rc = zmq_bind (xpub, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    void *xsub = zmq_socket (ctx, ZMQ_XSUB);
    assert (xsub);
    rc = zmq_bind (xsub, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    //  Create a publisher
    void *pub = zmq_socket (ctx, ZMQ_PUB);
    assert (pub);
    rc = zmq_connect (pub, "tcp://127.0.0.1:5561");
    assert (rc == 0);
    
    //  Create a subscriber
    void *sub = zmq_socket (ctx, ZMQ_SUB);
    assert (sub);
    rc = zmq_connect (sub, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    //  Subscribe for all messages.
    rc = zmq_setsockopt (sub, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    
    //  Pass the subscription upstream through the device
    char buff [32];
    rc = zmq_recv (xpub, buff, sizeof (buff), 0);
    assert (rc >= 0);
    rc = zmq_send (xsub, buff, rc, 0);
    assert (rc >= 0);
    
    //  Wait a bit till the subscription gets to the publisher
    msleep (SETTLE_TIME);
    
    //  Send an empty message
    rc = zmq_send (pub, NULL, 0, 0);
    assert (rc == 0);
    
    //  Pass the message downstream through the device
    rc = zmq_recv (xsub, buff, sizeof (buff), 0);
    assert (rc >= 0);
    rc = zmq_send (xpub, buff, rc, 0);
    assert (rc >= 0);
    
    //  Receive the message in the subscriber
    rc = zmq_recv (sub, buff, sizeof (buff), 0);
    assert (rc == 0);
    
    //  Clean up.
    rc = zmq_close (xpub);
    assert (rc == 0);
    rc = zmq_close (xsub);
    assert (rc == 0);
    rc = zmq_close (pub);
    assert (rc == 0);
    rc = zmq_close (sub);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_system (void)
{
    NSLog(@"- test_system");
    
    //  Check that we have local networking via ZeroMQ
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    void *dealer = zmq_socket (ctx, ZMQ_DEALER);
    if (zmq_bind (dealer, "tcp://127.0.0.1:5670") == -1) {
        printf ("E: Cannot find 127.0.0.1 -- your system does not have local\n");
        printf ("E: networking. Please fix this before running libzmq checks.\n");
        return -1;
    }
    //  Check that we can create 100 sockets
    int handle [50];
    int count;
    for (count = 0; count < 50; count++) {
        handle [count] = socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
        if (handle [count] == -1) {
            printf ("W: Only able to create %d sockets on this box\n", count);
            printf ("I: Tune your system to increase maximum allowed file handles\n");
#if defined (ZMQ_HAVE_OSX)
            printf ("I: On OS/X, run 'ulimit -n 1200' in bash\n");
#elif defined (ZMQ_HAVE_LINUX)
            printf ("I: On Linux, run 'ulimit -n 1200' in bash\n");
#endif
            return -1;
        }
    }
    //  Release the socket handles
    for (count = 0; count < 1000; count++) {
        close(handle[count]);
    }
    
    zmq_close(dealer);
    zmq_ctx_term(ctx);
    return 0;
}

int test_term_endpoint (void)
{
    NSLog(@"- test_term_endpoint");
    setup_test_environment();
    int rc;
    char buf[FILENAME_MAX+1];
    size_t buf_size;
    const char *ep = "tcp://127.0.0.1:5560";
    const char *ep_wc_tcp = "tcp://127.0.0.1:*";
#if !defined ZMQ_HAVE_WINDOWS && !defined ZMQ_HAVE_OPENVMS
    const char *ep_wc_ipc = "ipc://*";
#endif
#if defined ZMQ_HAVE_VMCI
    const char *ep_wc_vmci = "vmci://*:*";
#endif
    
    //  Create infrastructure.
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    void *push = zmq_socket (ctx, ZMQ_PUSH);
    assert (push);
    rc = zmq_bind (push, ep);
    assert (rc == 0);
    void *pull = zmq_socket (ctx, ZMQ_PULL);
    assert (pull);
    rc = zmq_connect (pull, ep);
    assert (rc == 0);
    
    //  Pass one message through to ensure the connection is established
    rc = zmq_send (push, "ABC", 3, 0);
    assert (rc == 3);
    rc = zmq_recv (pull, buf, sizeof (buf), 0);
    assert (rc == 3);
    
    //  Unbind the listening endpoint
    rc = zmq_unbind (push, ep);
    assert (rc == 0);
    
    //  Allow unbind to settle
    msleep (SETTLE_TIME);
    
    //  Check that sending would block (there's no outbound connection)
    rc = zmq_send (push, "ABC", 3, ZMQ_DONTWAIT);
    assert (rc == -1 && zmq_errno () == EAGAIN);
    
    //  Clean up
    rc = zmq_close (pull);
    assert (rc == 0);
    rc = zmq_close (push);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Create infrastructure
    ctx = zmq_ctx_new ();
    assert (ctx);
    push = zmq_socket (ctx, ZMQ_PUSH);
    assert (push);
    rc = zmq_connect (push, ep);
    assert (rc == 0);
    pull = zmq_socket (ctx, ZMQ_PULL);
    assert (pull);
    rc = zmq_bind (pull, ep);
    assert (rc == 0);
    
    //  Pass one message through to ensure the connection is established.
    rc = zmq_send (push, "ABC", 3, 0);
    assert (rc == 3);
    rc = zmq_recv (pull, buf, sizeof (buf), 0);
    assert (rc == 3);
    
    //  Disconnect the bound endpoint
    rc = zmq_disconnect (push, ep);
    assert (rc == 0);
    
    //  Allow disconnect to settle
    msleep (SETTLE_TIME);
    
    //  Check that sending would block (there's no inbound connections).
    rc = zmq_send (push, "ABC", 3, ZMQ_DONTWAIT);
    assert (rc == -1 && zmq_errno () == EAGAIN);
    
    //  Clean up.
    rc = zmq_close (pull);
    assert (rc == 0);
    rc = zmq_close (push);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Create infrastructure (wild-card binding)
    ctx = zmq_ctx_new ();
    assert (ctx);
    push = zmq_socket (ctx, ZMQ_PUSH);
    assert (push);
    rc = zmq_bind (push, ep_wc_tcp);
    assert (rc == 0);
    pull = zmq_socket(ctx, ZMQ_PULL);
    assert(pull);
#if !defined ZMQ_HAVE_WINDOWS && !defined ZMQ_HAVE_OPENVMS
    rc = zmq_bind (pull, ep_wc_ipc);
    assert (rc == 0);
#endif
#if defined ZMQ_HAVE_VMCI
    void *req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    rc = zmq_bind (req, ep_wc_vmci);
    assert (rc == 0);
#endif
    
    // Unbind sockets binded by wild-card address
    buf_size = sizeof(buf);
    rc = zmq_getsockopt (push, ZMQ_LAST_ENDPOINT, buf, &buf_size);
    assert (rc == 0);
    rc = zmq_unbind (push, buf);
    assert (rc == 0);
#if !defined ZMQ_HAVE_WINDOWS && !defined ZMQ_HAVE_OPENVMS
    buf_size = sizeof(buf);
    rc = zmq_getsockopt (pull, ZMQ_LAST_ENDPOINT, buf, &buf_size);
    assert (rc == 0);
    rc = zmq_unbind (pull, buf);
    assert (rc == 0);
#endif
#if defined ZMQ_HAVE_VMCI
    buf_size = sizeof(buf);
    rc = zmq_getsockopt (req, ZMQ_LAST_ENDPOINT, buf, &buf_size);
    assert (rc == 0);
    rc = zmq_unbind(req, buf);
    assert (rc == 0);
#endif
    
    //  Clean up.
    rc = zmq_close (pull);
    assert (rc == 0);
    rc = zmq_close (push);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    //  Create infrastructure (wild-card binding)
    ctx = zmq_ctx_new ();
    assert (ctx);
    push = zmq_socket (ctx, ZMQ_PUSH);
    assert (push);
    rc = zmq_bind (push, ep_wc_tcp);
    assert (rc == 0);
    pull = zmq_socket(ctx, ZMQ_PULL);
    assert(pull);
#if !defined ZMQ_HAVE_WINDOWS && !defined ZMQ_HAVE_OPENVMS
    rc = zmq_bind (pull, ep_wc_ipc);
    assert (rc == 0);
#endif
#if defined ZMQ_HAVE_VMCI
    req = zmq_socket (ctx, ZMQ_REQ);
    assert (req);
    rc = zmq_bind (req, ep_wc_vmci);
    assert (rc == 0);
#endif
    
    // Sockets binded by wild-card address can't be unbinded by wild-card address
    rc = zmq_unbind (push, ep_wc_tcp);
    assert (rc == -1 && zmq_errno () == ENOENT);
#if !defined ZMQ_HAVE_WINDOWS && !defined ZMQ_HAVE_OPENVMS
    rc = zmq_unbind (pull, ep_wc_ipc);
    assert (rc == -1 && zmq_errno () == ENOENT);
#endif
#if defined ZMQ_HAVE_VMCI
    rc = zmq_unbind (req, ep_wc_vmci);
    assert (rc == -1 && zmq_errno () == ENOENT);
#endif
    
    //  Clean up.
    rc = zmq_close (pull);
    assert (rc == 0);
    rc = zmq_close (push);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

void test_thread_safe_client_thread (void *client)
{
    char data = 0;
    for (int count = 0; count < 15000; count++) {
        int rc = zmq_send (client, &data, 1, 0);
        assert (rc == 1);
    }
    data = 1;
    int rc = zmq_send (client, &data, 1, 0);
    assert (rc == 1);
}

int test_thread_safe (void)
{
    NSLog(@"- test_thread_safe");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *server = zmq_socket (ctx, ZMQ_SERVER);
    int rc = zmq_bind (server, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *client = zmq_socket (ctx, ZMQ_CLIENT);
    int thread_safe;
    size_t size = sizeof (int);
    zmq_getsockopt (client, ZMQ_THREAD_SAFE, &thread_safe, &size);
    assert (thread_safe == 1);
    rc = zmq_connect (client, "tcp://127.0.0.1:5560");
    assert (rc == 0);
    
    void *t1 = zmq_threadstart (test_thread_safe_client_thread, client);
    void *t2 = zmq_threadstart (test_thread_safe_client_thread, client);
    
    char data;
    int threads_completed = 0;
    while (threads_completed < 2) {
        zmq_recv (server, &data, 1, 0);
        if (data == 1)
            threads_completed++;            //  Thread ended
    }
    zmq_threadclose (t1);
    zmq_threadclose (t2);
    
    rc = zmq_close (server);
    assert (rc == 0);
    
    rc = zmq_close (client);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_timeio (void)
{
    NSLog(@"- test_timeio");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *frontend = zmq_socket (ctx, ZMQ_DEALER);
    assert (frontend);
    int rc = zmq_bind (frontend, "tcp://127.0.0.1:6001");
    assert (rc == 0);
    
    //  Receive on disconnected socket returns immediately
    char buffer [32];
    rc = zmq_recv (frontend, buffer, 32, ZMQ_DONTWAIT);
    assert (rc == -1);
    assert (zmq_errno() == EAGAIN);
    
    //  Check whether receive timeout is honored
    int timeout = 250;
    rc = zmq_setsockopt (frontend, ZMQ_RCVTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
  //  void* stopwatch = zmq_stopwatch_start();
    rc = zmq_recv (frontend, buffer, 32, 0);
    assert (rc == -1);
    assert (zmq_errno () == EAGAIN || zmq_errno() == EINTR );
    //unsigned int elapsed = zmq_stopwatch_stop(stopwatch) / 1000;
    //assert (elapsed > 200 && elapsed < 300);
    
    //  Check that normal message flow works as expected
    void *backend = zmq_socket (ctx, ZMQ_DEALER);
    assert (backend);
    rc = zmq_connect (backend, "tcp://127.0.0.1:6001");
    assert (rc == 0);
    rc = zmq_setsockopt (backend, ZMQ_SNDTIMEO, &timeout, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_send (backend, "Hello", 5, 0);
    assert (rc == 5);
    rc = zmq_recv (frontend, buffer, 32, 0);
    assert (rc == 5);
    
    //  Clean-up
    rc = zmq_close (backend);
    assert (rc == 0);
    
    rc = zmq_close (frontend);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

void test_timers_sleep_ (long timeout_)
{
#if defined ZMQ_HAVE_WINDOWS
    Sleep (timeout_ > 0 ? timeout_ : INFINITE);
#elif defined ZMQ_HAVE_ANDROID
    usleep (timeout_ * 1000);
#else
    usleep (timeout_ * 1000);
#endif
}

void  test_timers_handler (int timer_id, void* arg)
{
    (void) timer_id;               //  Stop 'unused' compiler warnings
    *((bool *)arg) = true;
}

int  test_timers_sleep_and_execute(void *timers_)
{
    int timeout = zmq_timers_timeout (timers_);
    
    //  Sleep methods are inaccurate, so we sleep in a loop until time arrived
    while (timeout > 0) {
         test_timers_sleep_ (timeout);
        timeout = zmq_timers_timeout(timers_);
    }
    
    return zmq_timers_execute(timers_);
}

int test_timers (void)
{
    NSLog(@"- test_timers");
    setup_test_environment ();
    
    void* timers = zmq_timers_new ();
    assert (timers);
    
    bool timer_invoked = false;
    
    int timer_id = zmq_timers_add (timers, 100,  test_timers_handler, &timer_invoked);
    assert (timer_id);
    
    //  Timer should be invoked yet
    int rc = zmq_timers_execute (timers);
    assert (rc == 0);
    assert (!timer_invoked);
    
    //  Wait half the time and check again
     test_timers_sleep_ (zmq_timers_timeout (timers) / 2);
    rc = zmq_timers_execute (timers);
    assert (rc == 0);
    assert (!timer_invoked);
    
    // Wait until the end
    rc =  test_timers_sleep_and_execute (timers);
    assert (rc == 0);
    assert (timer_invoked);
    timer_invoked = false;
    
    //  Wait half the time and check again
    long timeout = zmq_timers_timeout (timers);
     test_timers_sleep_ (timeout / 2);
    rc = zmq_timers_execute (timers);
    assert (rc == 0);
    assert (!timer_invoked);
    
    // Reset timer and wait half of the time left
    rc = zmq_timers_reset (timers, timer_id);
     test_timers_sleep_ (timeout / 2);
    rc = zmq_timers_execute (timers);
    assert (rc == 0);
    assert (!timer_invoked);
    
    // Wait until the end
    rc =  test_timers_sleep_and_execute(timers);
    assert (rc == 0);
    assert (timer_invoked);
    timer_invoked = false;
    
    // reschedule
    zmq_timers_set_interval (timers, timer_id, 50);
    rc =  test_timers_sleep_and_execute(timers);
    assert (rc == 0);
    assert (timer_invoked);
    timer_invoked = false;
    
    // cancel timer
    timeout = zmq_timers_timeout (timers);
    zmq_timers_cancel (timers, timer_id);
     test_timers_sleep_ (timeout * 2);
    rc = zmq_timers_execute (timers);
    assert (rc == 0);
    assert (!timer_invoked);
    
    rc = zmq_timers_destroy (&timers);
    assert (rc == 0);
    
    return 0;
}

int test_udp_msg_send (zmq_msg_t *msg_, void *s_, const char* group_, const char* body_)
{
    int rc = zmq_msg_init_size (msg_, strlen (body_));
    if (rc != 0)
        return rc;
    
    memcpy (zmq_msg_data (msg_), body_, strlen (body_));
    
    rc = zmq_msg_set_group (msg_, group_);
    if (rc != 0) {
        zmq_msg_close (msg_);
        return rc;
    }
    
    rc = zmq_msg_send (msg_, s_, 0);
    
    zmq_msg_close (msg_);
    
    return rc;
}

int test_udp_msg_recv_cmp (zmq_msg_t *msg_, void *s_, const char* group_, const char* body_)
{
    int rc = zmq_msg_init (msg_);
    if (rc != 0)
        return -1;
    
    int recv_rc = zmq_msg_recv (msg_, s_, 0);
    if (recv_rc == -1) {
        zmq_msg_close(msg_);
        return -1;
    }
    
    if (strcmp (zmq_msg_group (msg_), group_) != 0)
    {
        zmq_msg_close (msg_);
        return -1;
    }
    
    char * body = (char*) malloc (sizeof(char) * (zmq_msg_size (msg_) + 1));
    memcpy (body, zmq_msg_data (msg_), zmq_msg_size (msg_));
    body [zmq_msg_size (msg_)] = '\0';
    
    if (strcmp (body, body_) != 0)
    {
        zmq_msg_close (msg_);
        free(body);
        return -1;
    }
    
    zmq_msg_close (msg_);
    free (body);
    return recv_rc;
}

int test_udp (void)
{
    NSLog(@"- test_udp");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    zmq_msg_t msg;
    
    void *radio = zmq_socket (ctx, ZMQ_RADIO);
    void *dish = zmq_socket (ctx, ZMQ_DISH);
    
    //  Connecting dish should fail
    int rc = zmq_connect (dish, "udp://127.0.0.1:5556");
    assert (rc == -1);
    
    rc = zmq_bind (dish, "udp://*:5556");
    assert (rc == 0);
    
    //  Bind radio should fail
    rc = zmq_bind (radio, "udp://*:5556");
    assert (rc == -1);
    
    rc = zmq_connect (radio, "udp://127.0.0.1:5556");
    assert (rc == 0);
    
    msleep (SETTLE_TIME);
    
    rc = zmq_join (dish, "TV");
    assert (rc == 0);
    
    rc = test_udp_msg_send (&msg, radio, "TV", "Friends");
    assert (rc != -1);
    
    rc = test_udp_msg_recv_cmp (&msg, dish, "TV", "Friends");
    assert (rc != -1);
    
    rc = zmq_close (dish);
    assert (rc == 0);
    
    rc = zmq_close (radio);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_unbind_wildcard (void)
{
    NSLog(@"- test_unbind_wildcard");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    int ipv6 = is_ipv6_available ();
    
    /* Address wildcard, IPv6 disabled */
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    void *sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    int rc = zmq_bind (sb, "tcp://*:5555");
    assert (rc == 0);
    
    char bindEndpoint[256];
    size_t endpoint_len = sizeof (bindEndpoint);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, bindEndpoint, &endpoint_len);
    assert (rc == 0);
    
    char connectEndpoint[256];
    
#ifdef ZMQ_HAVE_WINDOWS
    strcpy(connectEndpoint, "tcp://127.0.0.1:5555");
#else
    strcpy(connectEndpoint, bindEndpoint);
#endif
    
    rc = zmq_connect (sc, connectEndpoint);
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, connectEndpoint);
    assert (rc == 0);
    rc = zmq_unbind (sb, bindEndpoint);
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    /* Address wildcard, IPv6 enabled */
    sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (sb, "tcp://*:5556");
    assert (rc == 0);
    
    endpoint_len = sizeof (bindEndpoint);
    memset(bindEndpoint, 0, endpoint_len);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, bindEndpoint, &endpoint_len);
    assert (rc == 0);
    
#ifdef ZMQ_HAVE_WINDOWS
    if (ipv6)
        strcpy(connectEndpoint, "tcp://[::1]:5556");
    else
        strcpy(connectEndpoint, "tcp://127.0.0.1:5556");
#else
    strcpy(connectEndpoint, bindEndpoint);
#endif
    
    rc = zmq_connect (sc, connectEndpoint);
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, connectEndpoint);
    assert (rc == 0);
    rc = zmq_unbind (sb, bindEndpoint);
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    /* Port wildcard, IPv4 address, IPv6 disabled */
    sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:*");
    assert (rc == 0);
    
    char endpoint[256];
    endpoint_len = sizeof (endpoint);
    memset(endpoint, 0, endpoint_len);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, endpoint, &endpoint_len);
    assert (rc == 0);
    
    rc = zmq_connect (sc, endpoint);
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, endpoint);
    assert (rc == 0);
    rc = zmq_unbind (sb, endpoint);
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    /* Port wildcard, IPv4 address, IPv6 enabled */
    sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:*");
    assert (rc == 0);
    
    endpoint_len = sizeof (endpoint);
    memset(endpoint, 0, endpoint_len);
    rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, endpoint, &endpoint_len);
    assert (rc == 0);
    
    rc = zmq_connect (sc, endpoint);
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, endpoint);
    assert (rc == 0);
    rc = zmq_unbind (sb, endpoint);
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    if (ipv6) {
        /* Port wildcard, IPv6 address, IPv6 enabled */
        sb = zmq_socket (ctx, ZMQ_REP);
        assert (sb);
        sc = zmq_socket (ctx, ZMQ_REQ);
        assert (sc);
        
        rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
        assert (rc == 0);
        rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_bind (sb, "tcp://[::1]:*");
        assert (rc == 0);
        
        endpoint_len = sizeof (endpoint);
        memset(endpoint, 0, endpoint_len);
        rc = zmq_getsockopt (sb, ZMQ_LAST_ENDPOINT, endpoint, &endpoint_len);
        assert (rc == 0);
        
        rc = zmq_connect (sc, endpoint);
        assert (rc == 0);
        
        bounce (sb, sc);
        
        rc = zmq_disconnect (sc, endpoint);
        assert (rc == 0);
        rc = zmq_unbind (sb, endpoint);
        assert (rc == 0);
        
        rc = zmq_close (sc);
        assert (rc == 0);
        rc = zmq_close (sb);
        assert (rc == 0);
    }
    
    /* No wildcard, IPv4 address, IPv6 disabled */
    sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:5557");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5557");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5557");
    assert (rc == 0);
    rc = zmq_unbind (sb, "tcp://127.0.0.1:5557");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    /* No wildcard, IPv4 address, IPv6 enabled */
    sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    sc = zmq_socket (ctx, ZMQ_REQ);
    assert (sc);
    
    rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
    assert (rc == 0);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:5558");
    assert (rc == 0);
    rc = zmq_connect (sc, "tcp://127.0.0.1:5558");
    assert (rc == 0);
    
    bounce (sb, sc);
    
    rc = zmq_disconnect (sc, "tcp://127.0.0.1:5558");
    assert (rc == 0);
    rc = zmq_unbind (sb, "tcp://127.0.0.1:5558");
    assert (rc == 0);
    
    rc = zmq_close (sc);
    assert (rc == 0);
    rc = zmq_close (sb);
    assert (rc == 0);
    
    if (ipv6) {
        /* No wildcard, IPv6 address, IPv6 enabled */
        sb = zmq_socket (ctx, ZMQ_REP);
        assert (sb);
        sc = zmq_socket (ctx, ZMQ_REQ);
        assert (sc);
        
        rc = zmq_setsockopt (sb, ZMQ_IPV6, &ipv6, sizeof (int));
        assert (rc == 0);
        rc = zmq_setsockopt (sc, ZMQ_IPV6, &ipv6, sizeof (int));
        assert (rc == 0);
        
        rc = zmq_bind (sb, "tcp://[::1]:5559");
        assert (rc == 0);
        rc = zmq_connect (sc, "tcp://[::1]:5559");
        assert (rc == 0);
        
        bounce (sb, sc);
        
        rc = zmq_disconnect (sc, "tcp://[::1]:5559");
        assert (rc == 0);
        rc = zmq_unbind (sb, "tcp://[::1]:5559");
        assert (rc == 0);
        
        rc = zmq_close (sc);
        assert (rc == 0);
        rc = zmq_close (sb);
        assert (rc == 0);
    }
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}

int test_unbind_inproc (void)
{
    NSLog(@"- test_unbind_inproc");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    int rc = zmq_bind (sb, "inproc://a");
    assert (rc == 0);
    
    rc = zmq_unbind (sb, "inproc://a");
    assert (rc == 0);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0;
}
int test_xpub_nodrop (void)
{
    NSLog(@"- test_xpub_nodrop");
    setup_test_environment();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create a publisher
    void *pub = zmq_socket (ctx, ZMQ_PUB);
    assert (pub);
    
    int hwm = 2000;
    int rc = zmq_setsockopt(pub, ZMQ_SNDHWM, &hwm, 4);
    assert(rc == 0);
    
    rc = zmq_bind (pub, "inproc://soname");
    assert (rc == 0);
    
    //  set pub socket options
    int wait = 1;
    rc = zmq_setsockopt (pub, ZMQ_XPUB_NODROP, &wait, 4);
    assert (rc == 0);
    
    
    //  Create a subscriber
    void *sub = zmq_socket (ctx, ZMQ_SUB);
    assert (sub);
    rc = zmq_connect (sub, "inproc://soname");
    assert (rc == 0);
    
    //  Subscribe for all messages.
    rc = zmq_setsockopt (sub, ZMQ_SUBSCRIBE, "", 0);
    assert (rc == 0);
    
    int hwmlimit = hwm - 1;
    int send_count = 0;
    
    //  Send an empty message
    for (int i = 0; i < hwmlimit; i++) {
        rc = zmq_send (pub, NULL, 0, 0);
        assert (rc == 0);
        send_count++;
    }
    
    int recv_count = 0;
    do {
        //  Receive the message in the subscriber
        rc = zmq_recv (sub, NULL, 0, ZMQ_DONTWAIT);
        if (rc == -1)
            assert (errno == EAGAIN);
        else {
            assert (rc == 0);
            recv_count++;
        }
    }
    while (rc == 0);
    
    assert (send_count == recv_count);
    
    //  Now test real blocking behavior
    //  Set a timeout, default is infinite
    int timeout = 0;
    rc = zmq_setsockopt (pub, ZMQ_SNDTIMEO, &timeout, 4);
    assert (rc == 0);
    
    send_count = 0;
    recv_count = 0;
    hwmlimit = hwm;
    
    //  Send an empty message until we get an error, which must be EAGAIN
    while (zmq_send (pub, "", 0, 0) == 0)
        send_count++;
    assert (errno == EAGAIN);
    
    while (zmq_recv (sub, NULL, 0, ZMQ_DONTWAIT) == 0)
        recv_count++;
    assert (send_count == recv_count);
    
    //  Clean up.
    rc = zmq_close (pub);
    assert (rc == 0);
    rc = zmq_close (sub);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}

int test_xpub_welcome_msg (void)
{
    NSLog(@"- test_xpub_welcome_msg");
    setup_test_environment ();
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    //  Create a publisher
    void *pub = zmq_socket (ctx, ZMQ_XPUB);
    assert (pub);
    int rc = zmq_bind (pub, "inproc://soname");
    assert (rc == 0);
    
    //  set pub socket options
    rc = zmq_setsockopt (pub, ZMQ_XPUB_WELCOME_MSG, "W", 1);
    assert (rc == 0);
    
    //  Create a subscriber
    void *sub = zmq_socket (ctx, ZMQ_SUB);
    
    // Subscribe to the welcome message
    rc = zmq_setsockopt (sub, ZMQ_SUBSCRIBE, "W", 1);
    assert (rc == 0);
    
    assert (sub);
    rc = zmq_connect (sub, "inproc://soname");
    assert (rc == 0);
    
    char buffer[2];
    
    // Receive the welcome subscription
    rc = zmq_recv(pub, buffer, 2, 0);
    assert (rc == 2);
    assert (buffer [0] == 1);
    assert (buffer [1] == 'W');
    
    // Receive the welcome message
    rc = zmq_recv (sub, buffer, 1, 0);
    assert (rc == 1);
    assert (buffer [0] == 'W');
    
    //  Clean up.
    rc = zmq_close (pub);
    assert (rc == 0);
    rc = zmq_close (sub);
    assert (rc == 0);
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    return 0 ;
}


int test_zmq_pod_fd (void)
{
    NSLog(@"- test_zmq_pod_fd");
    struct addrinfo *addr, hint;
    hint.ai_flags=AI_NUMERICHOST;
    hint.ai_family=AF_INET;
    hint.ai_socktype=SOCK_DGRAM;
    hint.ai_protocol=IPPROTO_UDP;
    hint.ai_addrlen=0;
    hint.ai_canonname=NULL;
    hint.ai_addr=NULL;
    hint.ai_next=NULL;
    
    int rc = getaddrinfo ("127.0.0.1", "6650", &hint, &addr);
    assert (rc == 0);
    
    int recv_socket = socket (AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    assert (recv_socket != -1);
    
    int flag = 1;
    rc = setsockopt (recv_socket, SOL_SOCKET, SO_REUSEADDR, &flag, sizeof (int));
    assert (rc == 0);
    
    rc = bind (recv_socket, addr->ai_addr, addr->ai_addrlen);
    assert (rc == 0);
    
    void *ctx = zmq_ctx_new ();
    assert (ctx);
    
    void *sb = zmq_socket (ctx, ZMQ_REP);
    assert (sb);
    
    rc = zmq_bind (sb, "tcp://127.0.0.1:*");
    assert (rc == 0);
    
    zmq_pollitem_t pollitems [] = {
        { sb, 0, ZMQ_POLLIN, 0 },
        { NULL, recv_socket, ZMQ_POLLIN, 0 },
    };
    
    int send_socket = socket (AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    assert (send_socket != -1);
    
    char buf[10];
    memset (buf, 1, 10);
    
    rc = sendto (send_socket, buf, 10, 0, addr->ai_addr, addr->ai_addrlen);
    assert (rc >= 0);
    
    assert (zmq_poll (pollitems, 2, 1) == 1);
    assert ((pollitems [0].revents & ZMQ_POLLIN) == 0);
    assert (pollitems [1].revents & ZMQ_POLLIN);
    
    rc = zmq_close (sb);
    assert (rc == 0);
    
    rc = zmq_ctx_term (ctx);
    assert (rc == 0);
    
    close (send_socket);
    close (recv_socket);
    
    freeaddrinfo(addr);
    
    return 0 ;
}

#define NBR_WORKERS 10

static void *
rt_dealer_worker_task(void *args)
{
    void *context = zmq_ctx_new();
    void *worker = zmq_socket(context, ZMQ_DEALER);
    
#if (defined (WIN32))
    s_set_id(worker, (intptr_t)args);
#else
    s_set_id(worker);          //  Set a printable identity
#endif
    
    zmq_connect (worker, "tcp://localhost:5671");
    
    int total = 0;
    while (1) {
        //  Tell the broker we're ready for work
        s_sendmore(worker, "");
        s_send(worker, "Hi Boss");
        
        //  Get workload from broker, until finished
        free(s_recv(worker));     //  Envelope delimiter
        char *workload = s_recv(worker);
        //  .skip
        int finished = (strcmp(workload, "Fired!") == 0);
        free(workload);
        if (finished) {
            printf("Completed: %d tasks\n", total);
            break;
        }
        total++;
        
        //  Do some random work
        s_sleep(randof(500) + 1);
    }
    zmq_close(worker);
    zmq_ctx_destroy(context);
    return NULL;
}

//  .split main task
//  While this example runs in a single process, that is just to make
//  it easier to start and stop the example. Each thread has its own
//  context and conceptually acts as a separate process.

int rt_dealer_main(void)
{
    void *context = zmq_ctx_new();
    void *broker = zmq_socket(context, ZMQ_ROUTER);
    
    zmq_bind(broker, "tcp://*:5671");
    srandom((unsigned)time(NULL));
    
    int worker_nbr;
    for (worker_nbr = 0; worker_nbr < NBR_WORKERS; worker_nbr++) {
        pthread_t worker;
        pthread_create(&worker, NULL, rt_dealer_worker_task, (void *)(intptr_t)worker_nbr);
    }
    //  Run for five seconds and then tell workers to end
    int64_t end_time = s_clock() + 5000;
    int workers_fired = 0;
    while (1) {
        //  Next message gives us least recently used worker
        char *identity = s_recv(broker);
        s_sendmore(broker, identity);
        free(identity);
        free(s_recv(broker));     //  Envelope delimiter
        free(s_recv(broker));     //  Response from worker
        s_sendmore(broker, "");
        
        //  Encourage workers until it's time to fire them
        if (s_clock() < end_time)
            s_send(broker, "Work harder");
        else {
            s_send(broker, "Fired!");
            if (++workers_fired == NBR_WORKERS)
                break;
        }
    }
    zmq_close(broker);
    zmq_ctx_destroy(context);
    return 0;
}
//
//  Task ventilator
//  Binds PUSH socket to tcp://localhost:5557
//  Sends batch of tasks to workers via that socket
//


int task_ventilator (void)
{
    void *context = zmq_ctx_new ();
    
    //  Socket to send messages on
    void *sender = zmq_socket (context, ZMQ_PUSH);
    zmq_bind (sender, "tcp://*:5557");
    
    //  Socket to send start of batch message on
    void *sink = zmq_socket (context, ZMQ_PUSH);
    zmq_connect (sink, "tcp://localhost:5558");
    
 //   printf ("Press Enter when the workers are ready: ");
  //  getchar ();
    printf ("Sending tasks to workers…\n");
    
    //  The first message is "0" and signals start of batch
    s_send (sink, "0");
    
    //  Initialize random number generator
    srandom ((unsigned) time (NULL));
    
    //  Send 100 tasks
    int task_nbr;
    int total_msec = 0;     //  Total expected cost in msecs
    for (task_nbr = 0; task_nbr < 10000; task_nbr++) {
        int workload;
        //  Random workload from 1 to 100msecs
        workload = randof (100) + 1;
        total_msec += workload;
        char string [10];
        sprintf (string, "%d", workload);
        s_send (sender, string);
    }
    printf ("Total expected cost: %d msec\n", total_msec);
    sleep (1);              //  Give 0MQ time to deliver
    
    zmq_close (sink);
    zmq_close (sender);
    zmq_ctx_destroy (context);
    return 0;
}

int weather_main (void)
{
    //  Prepare our context and publisher
    void *context = zmq_ctx_new ();
    void *publisher = zmq_socket (context, ZMQ_PUB);
    int rc = zmq_bind (publisher, "tcp://127.0.0.1:5556");
    assert (rc == 0);
    
    //  Initialize random number generator
    srandom ((unsigned) time (NULL));
    while (1) {
        //  Get values that will fool the boss
        int zipcode, temperature, relhumidity;
        zipcode     = randof (100000);
        temperature = randof (215) - 80;
        relhumidity = randof (50) + 10;
        
        //  Send message to all subscribers
        char update [200];
        sprintf (update, "%05d %d %d", zipcode, temperature, relhumidity);
        s_send (publisher, update);
      //  NSLog(@"send weather");
        sleep(1);
    }
    zmq_close (publisher);
    zmq_ctx_destroy (context);
    return 0;
}
int mspoller_main (void)
{
    //  Connect to task ventilator
    void *context = zmq_ctx_new ();
    void *receiver = zmq_socket (context, ZMQ_PULL);
    zmq_connect (receiver, "tcp://localhost:5557");
    
    //  Connect to weather server
    void *subscriber = zmq_socket (context, ZMQ_SUB);
    zmq_connect (subscriber, "tcp://127.0.0.1:5556");
//    zmq_setsockopt (subscriber, ZMQ_SUBSCRIBE, "10001 ", 6);
    zmq_setsockopt( subscriber, ZMQ_SUBSCRIBE, "", 0 );
    //  Process messages from both sockets
    unsigned int loopCounter = 0;

    while (1) {
        char msg [256];
        zmq_pollitem_t items [] = {
            { receiver,   0, ZMQ_POLLIN, 0 },
            { subscriber, 0, ZMQ_POLLIN, 0 }
        };
        NSLog(@"poll cycle %d", loopCounter++);
        NSLog(@"before poll");
        zmq_poll (items, 2, -1);
        NSLog(@"after poll");
        if (items [0].revents & ZMQ_POLLIN) {
            memset( msg, 0, sizeof(msg));
            int size = zmq_recv (receiver, msg, 255, 0);
            if (size != -1) {
                NSString *s = [ NSString stringWithUTF8String:msg];
                NSLog(@"got ventillator message size %d, mes= %@", size, s);
                //  Process task
            }
        }
        if (items [1].revents & ZMQ_POLLIN) {
            memset( msg, 0, sizeof(msg));
            int size = zmq_recv (subscriber, msg, 255, 0);
            if (size != -1) {
                NSString *s = [ NSString stringWithUTF8String:msg];
                NSLog(@"got weather message size %d, mes= %@", size, s);
                //  Process weather update
            }
        }
    }
    zmq_close (subscriber);
    zmq_ctx_destroy (context);
    return 0;
}

@implementation ZeroMQTests

+(void) zmqPollerExample
{
    //  rt_dealer_main();
    dispatch_queue_t zmq1 = dispatch_queue_create("zm1", nil);
    dispatch_queue_t zmq2 = dispatch_queue_create("zm2", nil);
    dispatch_queue_t zmq3 = dispatch_queue_create("zm3", nil);
    
    dispatch_async(zmq1, ^{
        task_ventilator();
    });
    dispatch_async(zmq2, ^{
        weather_main();
    });
    dispatch_async(zmq3, ^{
        mspoller_main();
    });
}

+(void) zeroMQtests
{
   // Some examples from official ZeroMQ examples repository
   // rt_dealer_main();
   // [ZeroMQTests zmqPollerExample];
    
   // Official ZeroMQ tests

    test_zmq_pod_fd();
    test_xpub_welcome_msg();
    test_xpub_nodrop();
    test_unbind_wildcard();
    test_unbind_inproc();
    test_udp();
    test_timers();
    test_timeio(); // does not work on device
    test_thread_safe();
    // test_term_endpoint(); //uses IPC
    //test_system(); // calls abort()
    test_sub_forward();
    test_stream_timeout(); // does not work on device
    test_stream_exceeds_buffer();
    test_stream_empty();
    test_stream_disconnect();
    test_stream();
    test_srcfd();
    test_spec_router();
    test_spec_req();
    test_spec_rep();
    test_spec_pushpull(); // does not work on device
    test_spec_dealer();
    test_sodium();
    test_sockopt_hwm();
    test_shutdown_stress(); // does not work on device
    test_setsockopt();
    test_security_plain(); // does not work on device
    test_sequrity_null();  // does not work on device
    test_security_curve(); // does not work on device
    test_scatter_gather();
    test_router_mandatory_hwn();
    test_router_mandatory();
    test_router_handover();
    test_reqrep_tcp();
    test_reqrep_inproc();
//    test_reqrep_device_tipc(); // tipc not supported
    test_reqrep_device();
    test_req_correlate();
    test_radio_dish();
    test_pub_invert_matching();
    test_proxy_terminate();
    test_proxy_single_socket(); // does not work on device
    test_proxy();
    test_probe_route();
    test_poller();
//    test_pair_tipc(); // tipc not supported
    test_pair_tcp();
//    test_pair_ipc(); // ipc not supported
    test_pair_inproc();
    test_msg_flags();
    test_msg_ffn();
    test_monitor();
    test_metadata();
    test_many_sockets();
    test_last_endpoint();
    test_issue_566();
    //test_ipc_wildcard(); // ipc not supported
    test_iov();
    test_invalid_rep();
    test_inproc_connect();
    test_immediate(); // does not work on device
    test_hwm_pubsub();
    test_hwm();
    test_heartbeats(); // does not wo	rk on device
    test_getsockopt_memset();
    //test_fork(); // fork is not supported
    //test_filter_inproc(); // ipc not supported
    test_disconnect_inproc();
    test_diffserver();
    test_dgram();
    test_ctx_options();
    test_ctx_destroy();
    test_connect_rid();
    test_connect_resolve();
    //test_connect_delay_tipc(); // tipc not supported
    test_conflate();
    test_client_server();
    test_capabilities();
    test_bind_src_address();
    test_bind_after_connect_tcp();
    test_base85();
    test_atomics();
    test_ancillaries();
    //test_abstract_ipc(); // tipc not supported */
    NSLog(@"all tests ok");
}

+(void) testAll
{
    dispatch_async(dispatch_get_global_queue(0, 0), ^{
       [ ZeroMQTests zeroMQtests];
    });
}
@end
