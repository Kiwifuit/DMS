import { For, createEffect, createSignal } from "solid-js";

export default function NewServer() {
  // Fields
  const [serverName, setServerName] = createSignal("");
  const [serverDescription, setServerDescription] = createSignal("");
  const [serverType, setServerType] = createSignal("");
  const [serverVersions, setServerVersions] = createSignal<string[]>([]);
  const [selectedServerVersion, setSelectedServerVersion] =
    createSignal<string>("");

  // Supported Servers
  const supportedServers = [
    "Forge",
    "Neoforge",
    "Arclight",
    "Fabric",
    "Quilt",
    "Glowstone",
  ];

  // Server creator
  const createNewServer = (e: Event) => {
    e.preventDefault();

    console.log({
      name: serverName(),
      description: serverDescription(),
      type: serverType(),
      versions: {
        software: selectedServerVersion(),
        game: null,
      },
    });

    setTimeout(() => {
      window.location.href = "/";
    }, 10_000);
  };

  // Server versions updater
  createEffect(() => {
    const server_type = serverType();

    // Clear the versions array before grabbing the versions
    setServerVersions([]);

    if (server_type) {
      for (let i = 0; i <= 6; i++) {
        setServerVersions((versions) => {
          return [...versions, `1.20.${i}-${server_type}`];
        });
      }
    }
  });

  return (
    <div class="flex flex-grow items-center justify-center">
      <div class="rounded-xl bg-light-dashboard-body p-5 dark:bg-dark-dashboard-body">
        <h1 class="text-xl">Create New Server</h1>
        <form onSubmit={(e) => createNewServer(e)} class="mt-4 grid gap-2">
          <div class="flex gap-3">
            <label for="server-name">Server Name:</label>
            <input
              type="text"
              id="server-name"
              value={serverName()}
              onInput={(e) => setServerName(e.currentTarget.value)}
            />
          </div>
          <div class="grid gap-1">
            <label for="server-description">Server Description:</label>
            <textarea
              id="server-desc"
              cols="30"
              value={serverDescription()}
              onInput={(e) => setServerDescription(e.currentTarget.value)}
            ></textarea>
          </div>

          <div class="flex gap-3">
            <label for="software">Server:</label>
            <select
              name="software"
              id="software"
              value={serverType()}
              onChange={(e) => setServerType(e.currentTarget.value)}
              class="grow"
            >
              <option />
              <For each={supportedServers}>
                {(software) => (
                  <option value={software.toLowerCase()} class="font-sans">
                    {software}
                  </option>
                )}
              </For>
            </select>
          </div>

          <div class="flex gap-3">
            <label for="software-version">Server version:</label>
            <select
              name="software-version"
              id="software-version"
              disabled={serverType() === "" || serverVersions().length <= 0}
              value={selectedServerVersion()}
              onChange={(e) => setSelectedServerVersion(e.target.value)}
              class="grow"
            >
              <option>Latest</option>
              <For each={serverVersions()}>
                {(server_version) => (
                  <option
                    value={server_version.toLowerCase()}
                    class="font-sans"
                  >
                    {server_version}
                  </option>
                )}
              </For>
            </select>
          </div>

          <button type="submit" class="rounded bg-server-online">
            Create Server
          </button>
        </form>
      </div>
    </div>
  );
}
