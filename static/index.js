const localvideo = document.getElementById("localvideo");
const remotevideo = document.getElementById("remotevideo");
const start_btn = document.getElementById("start_btn");
const con_btn = document.getElementById("con_btn");

const LOCAL_ID = "59913ed3-79c5-48da-be60-6b384075d0ed";
const pc = new RTCPeerConnection({
  iceServers: [
    {
      urls: ["stun:stun1.l.google.com:19302", "stun:stun2.l.google.com:19302"],
    },
    {
      urls: ["turn:35.198.0.186:3478"],
      username: "cancer",
      credential: "cancer",
    },
  ],
  iceCandidatePoolSize: 10,
});
con_btn.addEventListener("click", () => {
  pc.setRemoteDescription(
    new RTCSessionDescription({
      sdp: `v=0
o=- 8547356856679850143 3 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE 0 1
a=msid-semantic: WMS
m=audio 31922 UDP/TLS/RTP/SAVPF 109 9 0 8 101
c=IN IP4 177.22.178.143
a=rtcp:9 IN IP4 0.0.0.0
a=candidate:841689039 1 udp 2122260223 10.0.2.16 47116 typ host generation 0 network-id 5 network-cost 10
a=candidate:3158867029 1 udp 2122187263 fec0::aa15:8cf2:a836:6d0d 43071 typ host generation 0 network-id 6 network-cost 10
a=candidate:841689039 1 udp 2122129151 10.0.2.16 50699 typ host generation 0 network-id 1 network-cost 900
a=candidate:3158867029 1 udp 2122056191 fec0::aa15:8cf2:a836:6d0d 35359 typ host generation 0 network-id 2 network-cost 900
a=candidate:842163049 1 udp 1685921535 177.22.178.143 31922 typ srflx raddr 10.0.2.16 rport 50699 generation 0 network-id 1 network-cost 900
a=candidate:842163049 1 udp 1686052607 177.22.178.143 31530 typ srflx raddr 10.0.2.16 rport 47116 generation 0 network-id 5 network-cost 10
a=candidate:4231669940 1 udp 1685858559 2804:2a4c:108e:1d35:4e28:7cf4:1d8c:bb9 46665 typ srflx raddr fec0::aa15:8cf2:a836:6d0d rport 35359 generation 0 network-id 2 network-cost 900
a=candidate:4231669940 1 udp 1685989631 2804:2a4c:108e:1d35:4e28:7cf4:1d8c:bb9 46727 typ srflx raddr fec0::aa15:8cf2:a836:6d0d rport 43071 generation 0 network-id 6 network-cost 10
a=ice-ufrag:21WM
a=ice-pwd:uB0xJvfOmbR7J0LN1JkgAzNh
a=ice-options:trickle renomination
a=fingerprint:sha-256 6D:07:88:81:73:70:49:B3:9A:17:EC:86:CD:AA:DF:B8:CD:3E:A0:8F:B7:C1:96:CC:0A:BA:D3:81:BB:2B:F6:FE
a=setup:active
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level
a=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid
a=recvonly
a=rtcp-mux
a=rtpmap:109 opus/48000/2
a=fmtp:109 minptime=10;useinbandfec=1
a=rtpmap:9 G722/8000
a=rtpmap:0 PCMU/8000
a=rtpmap:8 PCMA/8000
a=rtpmap:101 telephone-event/8000
m=video 9 UDP/TLS/RTP/SAVPF 120 124 121 125 126 127
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:21WM
a=ice-pwd:uB0xJvfOmbR7J0LN1JkgAzNh
a=ice-options:trickle renomination
a=fingerprint:sha-256 6D:07:88:81:73:70:49:B3:9A:17:EC:86:CD:AA:DF:B8:CD:3E:A0:8F:B7:C1:96:CC:0A:BA:D3:81:BB:2B:F6:FE
a=setup:active
a=mid:1
a=extmap:5 urn:ietf:params:rtp-hdrext:toffset
a=extmap:4 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:7 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:3 urn:ietf:params:rtp-hdrext:sdes:mid
a=recvonly
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:120 VP8/90000
a=rtcp-fb:120 goog-remb
a=rtcp-fb:120 transport-cc
a=rtcp-fb:120 ccm fir
a=rtcp-fb:120 nack
a=rtcp-fb:120 nack pli
a=rtpmap:124 rtx/90000
a=fmtp:124 apt=120
a=rtpmap:121 VP9/90000
a=rtcp-fb:121 goog-remb
a=rtcp-fb:121 transport-cc
a=rtcp-fb:121 ccm fir
a=rtcp-fb:121 nack
a=rtcp-fb:121 nack pli
a=rtpmap:125 rtx/90000
a=fmtp:125 apt=121
a=rtpmap:126 H264/90000
a=rtcp-fb:126 goog-remb
a=rtcp-fb:126 transport-cc
a=rtcp-fb:126 ccm fir
a=rtcp-fb:126 nack
a=rtcp-fb:126 nack pli
a=fmtp:126 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f
a=rtpmap:127 rtx/90000
a=fmtp:127 apt=126`,
      type: "answer",
    })
  );
});

const ws = new WebSocket("ws://localhost:3000/ws");
ws.addEventListener("open", () => {
  console.log("ws open");
  ws.send(
    JSON.stringify({
      type: "Connection",
      data: JSON.stringify({ id: LOCAL_ID, slave: true }),
    })
  );
});
ws.addEventListener("error", console.error);
ws.addEventListener("close", console.warn);
ws.addEventListener("message", async (e) => {
  console.log(e.data);
  const req = JSON.parse(e.data);
  const data = JSON.parse(req.data);

  switch (req.type) {
    case "DidMatch":
      const offer_desc = await pc.createOffer();
      await pc.setLocalDescription(offer_desc);
      const offer = {
        type: offer_desc.type,
        sdp: offer_desc.sdp,
      };
      ws.send(
        JSON.stringify({
          type: "WebRTCConRequest",
          data: {
            sender_id: LOCAL_ID,
            target_id: data.target_id,
            code: offer,
          },
        })
      );
      break;
    case "WebRTCConRequested":
      pc.setRemoteDescription(data.code);
      ws.send(
        JSON.stringify({
          type: "WebRTCPeerConnectionEstablished",
          data: {
            sender_id: LOCAL_ID,
            target_id: data.target_id,
          },
        })
      );
      break;
    case "WebRTCConAccepted":
      break;
    case "IceCandidateAdded":
      const ice_data = JSON.parse(req.data);
      pc.addIceCandidate(new RTCIceCandidate(ice_data));
      break;
    case "Error":
      console.error(req.data);
      break;
    default:
      break;
  }
});

start_btn.addEventListener("click", async () => {
  const local_media = await navigator.mediaDevices.getUserMedia({
    video: true,
    audio: true,
  });
  const remote_media = new MediaStream();

  local_media.getTracks().forEach((track) => pc.addTrack(track, local_media));

  pc.ontrack = (e) => {
    e.streams[0].getTracks().forEach((track) => remote_media.addTrack(track));
  };

  localvideo.srcObject = local_media;
  remotevideo.srcObject = remote_media;

  pc.onicecandidate = ({ candidate }) => {
    if (candidate) {
      const res = JSON.stringify({
        type: "AddIceCandidate",
        data: {
          id: LOCAL_ID,
          candidate: candidate,
        },
      });
      ws.send(res);
      // console.log(candidate + `\n ${res}`);
    }
  };

  const offer_desc = await pc.createOffer();
  await pc.setLocalDescription(offer_desc);

  console.log(offer_desc);

  // ws.close(undefined, LOCAL_ID);
  //   const offer = {
  //     type: offer_desc.type,
  //     sdp: offer_desc.sdp,
  //   };

  //   ws.send(
  //     JSON.stringify({
  //       type: "CodeResponse",
  //       data: JSON.stringify({ id: LOCAL_ID, code: offer }),
  //     })
  //   );
});
