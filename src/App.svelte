<script lang="ts">
  import { _ } from "svelte-i18n";
  import { Router, Route } from "svelte-navigator";
  import AppList from "./routes/AppList.svelte";
</script>

<Router>
  <Route path="/">
    <AppList />
  </Route>
</Router>
