<script setup>
import ButtonPrimary from "../Button/Primary.vue";
import { ref } from "vue";
import { ClipboardIcon } from "@heroicons/vue/24/outline";

const peerIdentifier = ref("v=0 o=client_user 123456789 2 IN IP4 192.168.1.10 s=WebRTC Audio Call c=IN IP4 192.168.1.10 t=0 0 a=group:BUNDLE audio a=msid-semantic: WMS  m=audio 50000 RTP/AVP 111 103 104 c=IN IP4 192.168.1.10 a=rtpmap:111 OPUS/48000/2 a=fmtp:111 minptime=10; useinbandfec=1 a=rtpmap:103 ISAC/16000 a=rtpmap:104 ISAC/32000 a=ice-ufrag:ufragrand a=ice-pwd:pwdrandompwd a=sendrecv a=mid:audio a=rtcp-mux a=rtcp:50001 IN IP4 192.168.1.10  a=candidate:1 1 udp 2130706432 192.168.1.10 50000 typ host generation 0 a=candidate:2 1 udp 1686052863 10.0.0.5:12345 typ srflx raddr 192.168.1.10 rport 50000 generation 0 a=candidate:3 1 tcp 1266014463 203.0.113.42 60000 typ relay raddr 192.168.1.10 rport 50000 generation 0 a=end-of-candidates");

const copyPeerIdentifier = () => {
    navigator.clipboard.writeText(peerIdentifier.value)
};
</script>

<template>
	<div class="h-full flex flex-col justify-center items-center">
		<div class="flex flex-col">
			<h2 class="font-semibold text-2xl">Peer-to-peer end-to-end encrypted.</h2>

			<div class="mt-4 relative flex w-full">
				<ButtonPrimary @click="copyPeerIdentifier" class="absolute right-1 top-1 py-2 px-4 -mt-1">
					<ClipboardIcon class="size-5" />
				</ButtonPrimary>

				<input
					type="text"
					readonly
					class="peer h-full w-full rounded border border-zinc-200 py-2 px-4 pr-20 text-sm focus:border-2 focus:border-zinc-500 focus:border-t-transparent transition-all"
					:value="peerIdentifier"
				/>
				<label
					class="pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-xs transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-zinc-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-zinc-200 after:transition-all peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:!border-zinc-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:!border-zinc-500"
				>
					P2P identifier
				</label>
			</div>
		</div>
	</div>
</template>
