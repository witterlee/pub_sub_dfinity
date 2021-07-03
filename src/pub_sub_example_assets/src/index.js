import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as pub_sub_example_idl, canisterId as pub_sub_example_id } from 'dfx-generated/pub_sub_example';

const agent = new HttpAgent();
const pub_sub_example = Actor.createActor(pub_sub_example_idl, { agent, canisterId: pub_sub_example_id });

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  const greeting = await pub_sub_example.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
