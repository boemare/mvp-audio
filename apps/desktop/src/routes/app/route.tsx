import { createFileRoute, Outlet, useLocation } from "@tanstack/react-router";

import { TooltipProvider } from "@hypr/ui/components/ui/tooltip";

import { useConfigSideEffects } from "../../config/use-config";
import { DictationProvider } from "../../contexts/dictation";
import { ListenerProvider } from "../../contexts/listener";
import { isExtHostPath } from "../../utils/ext-host";

export const Route = createFileRoute("/app")({
  component: Component,
  loader: async ({ context: { listenerStore } }) => {
    return { listenerStore: listenerStore! };
  },
});

function Component() {
  const { listenerStore } = Route.useLoaderData();
  const location = useLocation();
  const isExtHost = isExtHostPath(location.pathname);

  if (isExtHost) {
    return (
      <TooltipProvider>
        <Outlet />
      </TooltipProvider>
    );
  }

  return (
    <TooltipProvider>
      <ListenerProvider store={listenerStore}>
        <DictationProvider>
          <Outlet />
          <SideEffects />
        </DictationProvider>
      </ListenerProvider>
    </TooltipProvider>
  );
}

function SideEffects() {
  useConfigSideEffects();

  return null;
}
