import { Title } from "@solidjs/meta";
import { sum } from "~/backend";

export default function Test() {
  return (
    <main>
      <Title>This should be visible if the component is rendered</Title>
      <p>Test {getVersionDummy()}</p>
    </main>
  );
}

function getVersionDummy(): string {
  console.log("this runs");
  return sum(1, 3).toString();
}
