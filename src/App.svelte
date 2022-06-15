<script>
  import { _ } from "svelte-i18n";
  import { Router, Route } from "svelte-navigator";
  import AppList from "./routes/AppList.svelte";
  import DeviceList from "./routes/DeviceList.svelte";

  import createHashHistory from "./logic/hashHistory";
  const hash = createHashHistory();
</script>

<Router primary={false} history={hash}>
  <Route path="/devices">
    <DeviceList />
  </Route>
  <Route path="/">
    <AppList />
  </Route>
</Router>
