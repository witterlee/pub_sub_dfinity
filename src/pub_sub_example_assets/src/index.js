import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory as publish_idl, canisterId as publish_id } from 'dfx-generated/publish';
import { idlFactory as subscriber_idl, canisterId as subscriber_id } from 'dfx-generated/subscriber';

const agent = new HttpAgent();
const publishActor = Actor.createActor(publish_idl, { agent, canisterId: publish_id });
const subscriberActor = Actor.createActor(subscriber_idl, { agent, canisterId: subscriber_id });

document.getElementById("clickSubscribeBtn").addEventListener("click", async () => {
  const publishCanisterId= Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai");
  const result = await subscriberActor.setup_subscribe(publishCanisterId);

  document.getElementById("greeting").innerText =JSON.stringify(result);
});


document.getElementById("clickTradeBtn").addEventListener("click", async () => {
  
  const greeting = await publishActor.greet(name);

  document.getElementById("greeting").innerText = greeting;
});

//dfx canister call subscriber setup_subscribe '((principal "ryjl3-tyaaa-aaaaa-aaaba-cai"))'