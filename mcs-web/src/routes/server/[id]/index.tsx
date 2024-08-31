import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();

  return (
    <main id="dashboard-ui">
      <Title>{params.id} dashboard</Title>
      <DashboardNavBar />
      <div class="col-start-2 ml-5">
        <h1 class="text-4xl font-bold">Dashboard</h1>
      </div>
    </main>
  );
}