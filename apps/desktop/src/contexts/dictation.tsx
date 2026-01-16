import React, { createContext, useContext, useEffect, useRef } from "react";
import { useStore } from "zustand";
import { useShallow } from "zustand/shallow";

import {
  createDictationStore,
  type DictationStore,
} from "../store/zustand/dictation";

const DictationContext = createContext<DictationStore | null>(null);

export const DictationProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const storeRef = useRef<DictationStore | null>(null);

  if (!storeRef.current) {
    storeRef.current = createDictationStore();
  }

  useEffect(() => {
    const store = storeRef.current;
    if (!store) return;

    void store.getState().init();

    return () => {
      store.getState().cleanup();
    };
  }, []);

  return (
    <DictationContext.Provider value={storeRef.current}>
      {children}
    </DictationContext.Provider>
  );
};

export const useDictation = <T,>(
  selector: Parameters<
    typeof useStore<ReturnType<typeof createDictationStore>, T>
  >[1],
) => {
  const store = useContext(DictationContext);

  if (!store) {
    throw new Error("'useDictation' must be used within a 'DictationProvider'");
  }

  return useStore(store, useShallow(selector));
};

export const useDictationStore = () => {
  const store = useContext(DictationContext);

  if (!store) {
    throw new Error(
      "'useDictationStore' must be used within a 'DictationProvider'",
    );
  }

  return store;
};
