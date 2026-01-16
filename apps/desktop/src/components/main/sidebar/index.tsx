import { useQuery } from "@tanstack/react-query";
import { platform } from "@tauri-apps/plugin-os";
import {
  AxeIcon,
  BookOpenIcon,
  ClockIcon,
  FileTextIcon,
  PanelLeftCloseIcon,
} from "lucide-react";
import { lazy, Suspense, useState } from "react";

import { Button } from "@hypr/ui/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@hypr/ui/components/ui/tooltip";
import { cn } from "@hypr/utils";

import { useSearch } from "../../../contexts/search/ui";
import { useShell } from "../../../contexts/shell";
import { commands } from "../../../types/tauri.gen";
import { TrafficLights } from "../../window/traffic-lights";
import { DictationHistoryView } from "./dictation-history";
import { DictionaryView } from "./dictionary";
import { ProfileSection } from "./profile";
import { SearchResults } from "./search";
import { TimelineView } from "./timeline";
import { ToastArea } from "./toast";

const DevtoolView = lazy(() =>
  import("./devtool").then((m) => ({ default: m.DevtoolView })),
);

type SidebarView = "notes" | "history" | "dictionary";

export function LeftSidebar() {
  const { leftsidebar } = useShell();
  const { query } = useSearch();
  const [isProfileExpanded, setIsProfileExpanded] = useState(false);
  const [activeView, setActiveView] = useState<SidebarView>("notes");
  const isLinux = platform() === "linux";

  const { data: showDevtoolButton = false } = useQuery({
    queryKey: ["show_devtool"],
    queryFn: () => commands.showDevtool(),
  });

  const showSearchResults = query.trim() !== "";

  const renderContent = () => {
    if (leftsidebar.showDevtool) {
      return (
        <Suspense fallback={null}>
          <DevtoolView />
        </Suspense>
      );
    }

    if (showSearchResults) {
      return <SearchResults />;
    }

    switch (activeView) {
      case "notes":
        return <TimelineView />;
      case "history":
        return <DictationHistoryView />;
      case "dictionary":
        return <DictionaryView />;
    }
  };

  return (
    <div className="h-full w-[280px] flex flex-col overflow-hidden shrink-0 gap-1">
      <header
        data-tauri-drag-region
        className={cn([
          "flex flex-row items-center",
          "w-full h-9 py-1",
          isLinux ? "pl-3 justify-between" : "pl-[72px] justify-end",
          "shrink-0",
          "rounded-xl bg-neutral-50",
        ])}
      >
        {isLinux && <TrafficLights />}
        <div className="flex items-center">
          {showDevtoolButton && (
            <Button
              size="icon"
              variant="ghost"
              onClick={leftsidebar.toggleDevtool}
            >
              <AxeIcon size={16} />
            </Button>
          )}
          <Button
            size="icon"
            variant="ghost"
            onClick={leftsidebar.toggleExpanded}
          >
            <PanelLeftCloseIcon size={16} />
          </Button>
        </div>
      </header>

      <div className="flex flex-col flex-1 overflow-hidden gap-1">
        <ViewSwitcher activeView={activeView} onViewChange={setActiveView} />
        <div className="flex-1 min-h-0 overflow-hidden relative">
          {renderContent()}
          {!leftsidebar.showDevtool && (
            <ToastArea isProfileExpanded={isProfileExpanded} />
          )}
        </div>
        <div className="relative z-30">
          <ProfileSection onExpandChange={setIsProfileExpanded} />
        </div>
      </div>
    </div>
  );
}

function ViewSwitcher({
  activeView,
  onViewChange,
}: {
  activeView: SidebarView;
  onViewChange: (view: SidebarView) => void;
}) {
  const views: { id: SidebarView; icon: typeof FileTextIcon; label: string }[] =
    [
      { id: "notes", icon: FileTextIcon, label: "Notes" },
      { id: "history", icon: ClockIcon, label: "Dictation History" },
      { id: "dictionary", icon: BookOpenIcon, label: "Dictionary" },
    ];

  return (
    <div className="flex items-center gap-1 px-2 py-1 bg-neutral-50 rounded-xl">
      {views.map((view, index) => (
        <div key={view.id} className="flex items-center">
          {index === 1 && <div className="w-px h-4 bg-neutral-200 mx-1" />}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                size="icon"
                variant="ghost"
                className={cn([
                  "h-7 w-7",
                  activeView === view.id && "bg-neutral-200",
                ])}
                onClick={() => onViewChange(view.id)}
              >
                <view.icon size={14} />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="bottom" className="text-xs">
              {view.label}
            </TooltipContent>
          </Tooltip>
        </div>
      ))}
    </div>
  );
}
