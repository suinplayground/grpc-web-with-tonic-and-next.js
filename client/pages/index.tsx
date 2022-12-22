import {
  createGrpcWebTransport,
  createPromiseClient,
} from "@bufbuild/connect-web";
import type { PartialMessage } from "@bufbuild/protobuf";
import { type FormEvent, useState } from "react";
import { Greeter } from "../services/greeting_connectweb";
// 生成されたコードには型情報もある
import type { GreetingMessage, Person } from "../services/greeting_pb";

// gRPCクライアントの初期化
const transport = createGrpcWebTransport({
  baseUrl: "http://localhost:50051",
});
const client = createPromiseClient(Greeter, transport);

export default function Home() {
  const [name, setName] = useState("");
  const [text, setText] = useState("");

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    // リクエストメッセージのオブジェクトはPartialMessageを使うと取れます
    const person: PartialMessage<Person> = { name };
    // gRPCメソッドを呼び出す
    const greetingMessage: GreetingMessage = await client.sayHello(person);
    console.log("greetingMessage: ", greetingMessage);
    setText(greetingMessage.text);
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <input
          placeholder="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button type="submit">submit</button>
      </form>
      <div>{text}</div>
    </>
  );
}
