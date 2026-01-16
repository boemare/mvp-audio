import { formatDistanceToNow } from "date-fns";
import { MicIcon } from "lucide-react";
import { useState } from "react";

import { cn } from "@hypr/utils";

import * as main from "../../../../store/tinybase/store/main";

export function DictationHistoryView() {
  const history = useDictationHistory();

  return (
    <div className={cn(["flex flex-col h-full", "bg-neutral-50 rounded-xl"])}>
      <div
        className={cn(["sticky top-0 z-10", "bg-neutral-50 pl-3 pr-1 py-2"])}
      >
        <div className="text-base font-bold text-neutral-900">
          Dictation History
        </div>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide">
        {history.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="divide-y divide-neutral-100">
            {history.map((item) => (
              <DictationHistoryItem key={item.id} item={item} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
      <div className="w-12 h-12 rounded-full bg-neutral-100 flex items-center justify-center mb-3">
        <MicIcon size={24} className="text-neutral-400" />
      </div>
      <p className="text-sm font-medium text-neutral-900 mb-1">
        No dictations yet
      </p>
      <p className="text-xs text-neutral-500">
        Press Ctrl+Shift+D to start dictating
      </p>
    </div>
  );
}

interface DictationHistoryItemData {
  id: string;
  created_at: string;
  raw_text: string;
  processed_text: string;
  duration_ms: number;
  target_app?: string;
}

function DictationHistoryItem({ item }: { item: DictationHistoryItemData }) {
  const [expanded, setExpanded] = useState(false);

  return (
    <button
      type="button"
      className={cn([
        "w-full text-left p-3",
        "hover:bg-neutral-100/50 transition-colors",
        "cursor-pointer",
      ])}
      onClick={() => setExpanded(!expanded)}
    >
      <div className="flex items-center justify-between mb-1">
        <span className="text-xs text-neutral-500">
          {formatDistanceToNow(new Date(item.created_at), { addSuffix: true })}
        </span>
        <span className="text-xs text-neutral-400">
          {(item.duration_ms / 1000).toFixed(1)}s
        </span>
      </div>
      <p
        className={cn([
          "text-sm text-neutral-700",
          !expanded && "line-clamp-2",
        ])}
      >
        {item.processed_text}
      </p>
      {item.target_app && (
        <span className="text-xs text-neutral-400 mt-1 block">
          Sent to: {item.target_app}
        </span>
      )}
    </button>
  );
}

function useDictationHistory(): DictationHistoryItemData[] {
  const store = main.UI.useStore(main.STORE_ID);
  const table = main.UI.useTable("dictation_history" as never, main.STORE_ID);

  if (!store || !table) {
    return [];
  }

  const items: DictationHistoryItemData[] = [];

  for (const [id, row] of Object.entries(table)) {
    items.push({
      id,
      created_at: String(row.created_at ?? ""),
      raw_text: String(row.raw_text ?? ""),
      processed_text: String(row.processed_text ?? ""),
      duration_ms: Number(row.duration_ms ?? 0),
      target_app: row.target_app ? String(row.target_app) : undefined,
    });
  }

  return items.sort(
    (a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );
}
