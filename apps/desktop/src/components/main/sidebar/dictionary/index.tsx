import { BookOpenIcon, PlusIcon, Trash2Icon } from "lucide-react";
import { useState } from "react";

import { Button } from "@hypr/ui/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@hypr/ui/components/ui/dialog";
import { Input } from "@hypr/ui/components/ui/input";
import { Label } from "@hypr/ui/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@hypr/ui/components/ui/select";
import { Textarea } from "@hypr/ui/components/ui/textarea";
import { cn } from "@hypr/utils";

import * as main from "../../../../store/tinybase/store/main";

export function DictionaryView() {
  const terms = useDictionaryTerms();
  const [isAddOpen, setIsAddOpen] = useState(false);

  return (
    <div className={cn(["flex flex-col h-full", "bg-neutral-50 rounded-xl"])}>
      <div
        className={cn([
          "sticky top-0 z-10",
          "bg-neutral-50 pl-3 pr-1 py-2",
          "flex items-center justify-between",
        ])}
      >
        <div className="text-base font-bold text-neutral-900">Dictionary</div>
        <Dialog open={isAddOpen} onOpenChange={setIsAddOpen}>
          <DialogTrigger asChild>
            <Button size="icon" variant="ghost" className="h-7 w-7">
              <PlusIcon size={14} />
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Add Dictionary Term</DialogTitle>
            </DialogHeader>
            <AddTermForm onSuccess={() => setIsAddOpen(false)} />
          </DialogContent>
        </Dialog>
      </div>

      <div className="flex-1 overflow-y-auto scrollbar-hide">
        {terms.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="space-y-1 p-2">
            {terms.map((term) => (
              <DictionaryTermCard key={term.id} term={term} />
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
        <BookOpenIcon size={24} className="text-neutral-400" />
      </div>
      <p className="text-sm font-medium text-neutral-900 mb-1">
        No dictionary terms
      </p>
      <p className="text-xs text-neutral-500">
        Add terms to improve transcription accuracy
      </p>
    </div>
  );
}

interface DictionaryTermData {
  id: string;
  term: string;
  type: "glossary" | "replacement";
  replacement?: string;
  context?: string;
}

function DictionaryTermCard({ term }: { term: DictionaryTermData }) {
  const store = main.UI.useStore(main.STORE_ID);

  const handleDelete = () => {
    if (store) {
      store.delRow("dictionary_terms" as never, term.id);
    }
  };

  return (
    <div
      className={cn([
        "p-2 rounded-lg",
        "bg-white border border-neutral-100",
        "group relative",
      ])}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="font-medium text-sm text-neutral-900 truncate">
              {term.term}
            </span>
            <span
              className={cn([
                "text-[10px] px-1.5 py-0.5 rounded-full",
                term.type === "glossary"
                  ? "bg-blue-50 text-blue-600"
                  : "bg-amber-50 text-amber-600",
              ])}
            >
              {term.type}
            </span>
          </div>
          {term.type === "replacement" && term.replacement && (
            <p className="text-xs text-neutral-500 mt-0.5">
              → {term.replacement}
            </p>
          )}
          {term.context && (
            <p className="text-xs text-neutral-400 mt-0.5 line-clamp-1">
              {term.context}
            </p>
          )}
        </div>
        <Button
          size="icon"
          variant="ghost"
          className="h-6 w-6 opacity-0 group-hover:opacity-100 transition-opacity shrink-0"
          onClick={handleDelete}
        >
          <Trash2Icon size={12} className="text-neutral-400" />
        </Button>
      </div>
    </div>
  );
}

function AddTermForm({ onSuccess }: { onSuccess: () => void }) {
  const store = main.UI.useStore(main.STORE_ID);
  const [term, setTerm] = useState("");
  const [type, setType] = useState<"glossary" | "replacement">("glossary");
  const [replacement, setReplacement] = useState("");
  const [context, setContext] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!store || !term.trim()) return;

    const id = crypto.randomUUID();
    store.setRow("dictionary_terms" as never, id, {
      user_id: "",
      term: term.trim(),
      type,
      replacement: type === "replacement" ? replacement.trim() : "",
      context: context.trim(),
    } as never);

    setTerm("");
    setType("glossary");
    setReplacement("");
    setContext("");
    onSuccess();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="term">Term</Label>
        <Input
          id="term"
          value={term}
          onChange={(e) => setTerm(e.target.value)}
          placeholder="e.g., Hyprnote"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="type">Type</Label>
        <Select
          value={type}
          onValueChange={(v) => setType(v as "glossary" | "replacement")}
        >
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="glossary">Glossary</SelectItem>
            <SelectItem value="replacement">Replacement</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {type === "replacement" && (
        <div className="space-y-2">
          <Label htmlFor="replacement">Replace with</Label>
          <Input
            id="replacement"
            value={replacement}
            onChange={(e) => setReplacement(e.target.value)}
            placeholder="e.g., HyprNote"
          />
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="context">Context (optional)</Label>
        <Textarea
          id="context"
          value={context}
          onChange={(e) => setContext(e.target.value)}
          placeholder="Additional context for better transcription"
          rows={2}
        />
      </div>

      <div className="flex justify-end">
        <Button type="submit" disabled={!term.trim()}>
          Add Term
        </Button>
      </div>
    </form>
  );
}

function useDictionaryTerms(): DictionaryTermData[] {
  const store = main.UI.useStore(main.STORE_ID);
  const table = main.UI.useTable("dictionary_terms" as never, main.STORE_ID);

  if (!store || !table) {
    return [];
  }

  const items: DictionaryTermData[] = [];

  for (const [id, row] of Object.entries(table)) {
    items.push({
      id,
      term: String(row.term ?? ""),
      type: (row.type === "replacement" ? "replacement" : "glossary") as
        | "glossary"
        | "replacement",
      replacement: row.replacement ? String(row.replacement) : undefined,
      context: row.context ? String(row.context) : undefined,
    });
  }

  return items.sort((a, b) => a.term.localeCompare(b.term));
}
