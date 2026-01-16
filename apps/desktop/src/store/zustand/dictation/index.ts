import { createStore } from "zustand";
import { create as mutate } from "mutative";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

import {
  commands as dictationCommands,
  events as dictationEvents,
  type DictationPhase,
  type DictationResult,
} from "@hypr/plugin-dictation";
import {
  commands as hotkeyCommands,
  events as hotkeyEvents,
  type HotkeyAction,
} from "@hypr/plugin-hotkeys";
import { commands as pasteCommands } from "@hypr/plugin-paste";
import { commands as sfxCommands } from "@hypr/plugin-sfx";

export type DictationState = {
  phase: DictationPhase;
  audioLevels: number[];
  lastResult: DictationResult | null;
  eventUnlisteners: (() => void)[];
  hotkeyId: string;
};

export type DictationActions = {
  init: () => Promise<void>;
  cleanup: () => void;
  startDictation: () => Promise<void>;
  stopDictation: () => Promise<void>;
  cancelDictation: () => Promise<void>;
  lockDictation: () => Promise<void>;
  unlockDictation: () => Promise<void>;
};

type Store = DictationState & DictationActions;

const DICTATION_HOTKEY_ID = "dictation-main";
const DEFAULT_HOTKEY_KEYS = ["Control", "Shift", "D"];

const initialState: DictationState = {
  phase: "idle",
  audioLevels: [],
  lastResult: null,
  eventUnlisteners: [],
  hotkeyId: DICTATION_HOTKEY_ID,
};

export type DictationStore = ReturnType<typeof createDictationStore>;

export const createDictationStore = () => {
  return createStore<Store>((set, get) => ({
    ...initialState,

    init: async () => {
      const unlisteners: (() => void)[] = [];

      const amplitudeUnlisten = await dictationEvents.dictationAmplitudeEvent.listen(
        ({ payload }) => {
          set((state) =>
            mutate(state, (draft) => {
              draft.audioLevels = payload.levels;
            }),
          );
        },
      );
      unlisteners.push(amplitudeUnlisten);

      const stateUnlisten = await dictationEvents.dictationStateEvent.listen(
        ({ payload }) => {
          set((state) =>
            mutate(state, (draft) => {
              draft.phase = payload.state;
            }),
          );
        },
      );
      unlisteners.push(stateUnlisten);

      const completeUnlisten = await dictationEvents.dictationCompleteEvent.listen(
        ({ payload }) => {
          set((state) =>
            mutate(state, (draft) => {
              draft.lastResult = payload.result;
              draft.phase = "idle";
            }),
          );
        },
      );
      unlisteners.push(completeUnlisten);

      const hotkeyUnlisten = await hotkeyEvents.hotkeyActionEvent.listen(
        async ({ payload }) => {
          if (payload.hotkey_id !== DICTATION_HOTKEY_ID) return;

          const action = payload.action as HotkeyAction;

          switch (action) {
            case "activate":
              await get().startDictation();
              break;
            case "deactivate":
              await get().stopDictation();
              break;
            case "lock":
              await get().lockDictation();
              break;
            case "unlock":
              await get().unlockDictation();
              break;
          }
        },
      );
      unlisteners.push(hotkeyUnlisten);

      set((state) =>
        mutate(state, (draft) => {
          draft.eventUnlisteners = unlisteners;
        }),
      );

      await hotkeyCommands.registerHotkey({
        id: DICTATION_HOTKEY_ID,
        keys: DEFAULT_HOTKEY_KEYS,
        enable_lock: true,
        lock_tap_count: 2,
        unlock_tap_count: 3,
        tap_timeout_ms: 400,
      });

      await hotkeyCommands.startListening();
    },

    cleanup: () => {
      const state = get();
      state.eventUnlisteners.forEach((fn) => fn());
      set((s) =>
        mutate(s, (draft) => {
          draft.eventUnlisteners = [];
        }),
      );

      void hotkeyCommands.unregisterHotkey(DICTATION_HOTKEY_ID);
      void hotkeyCommands.stopListening();
    },

    startDictation: async () => {
      const state = get();
      if (state.phase !== "idle") return;

      set((s) =>
        mutate(s, (draft) => {
          draft.phase = "recording";
        }),
      );

      await sfxCommands.play("StartRecording");
      const floatingBar = await WebviewWindow.getByLabel("floating-bar");
      if (floatingBar) {
        await floatingBar.show();
      }
      await dictationCommands.startDictation({ language: null });
    },

    stopDictation: async () => {
      const state = get();
      if (state.phase !== "recording" && state.phase !== "locked") return;

      set((s) =>
        mutate(s, (draft) => {
          draft.phase = "processing";
        }),
      );

      try {
        const result = await dictationCommands.stopDictation();

        if (result.status === "ok") {
          const dictationResult = result.data;

          set((s) =>
            mutate(s, (draft) => {
              draft.lastResult = dictationResult;
            }),
          );

          await pasteCommands.injectText(dictationResult.processed_text);
        }
      } finally {
        set((s) =>
          mutate(s, (draft) => {
            draft.phase = "idle";
            draft.audioLevels = [];
          }),
        );

        const floatingBar = await WebviewWindow.getByLabel("floating-bar");
        if (floatingBar) {
          await floatingBar.hide();
        }
        await sfxCommands.play("StopRecording");
      }
    },

    cancelDictation: async () => {
      const state = get();
      if (state.phase === "idle") return;

      await dictationCommands.cancelDictation();

      set((s) =>
        mutate(s, (draft) => {
          draft.phase = "idle";
          draft.audioLevels = [];
        }),
      );

      const floatingBar = await WebviewWindow.getByLabel("floating-bar");
      if (floatingBar) {
        await floatingBar.hide();
      }
    },

    lockDictation: async () => {
      const state = get();
      if (state.phase !== "recording") return;

      set((s) =>
        mutate(s, (draft) => {
          draft.phase = "locked";
        }),
      );
    },

    unlockDictation: async () => {
      const state = get();
      if (state.phase !== "locked") return;

      await get().stopDictation();
    },
  }));
};
