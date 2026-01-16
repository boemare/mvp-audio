import { createFileRoute } from "@tanstack/react-router";
import { motion } from "motion/react";
import { useEffect, useState } from "react";

import { cn } from "@hypr/utils";

import { useDictation } from "../../contexts/dictation";

export const Route = createFileRoute("/app/floating-bar")({
  component: FloatingBarWindow,
});

function FloatingBarWindow() {
  const { phase, audioLevels } = useDictation((state) => ({
    phase: state.phase,
    audioLevels: state.audioLevels,
  }));

  const isRecording = phase === "recording" || phase === "locked";
  const isProcessing = phase === "processing";

  const dimensions = {
    idle: { width: 39, height: 8 },
    recording: { width: 84, height: 29 },
    processing: { width: 84, height: 29 },
  };

  const currentDimensions =
    isRecording || isProcessing ? dimensions.recording : dimensions.idle;

  return (
    <div
      data-tauri-drag-region
      className="w-screen h-screen bg-transparent flex items-center justify-center"
    >
      <motion.div
        animate={currentDimensions}
        transition={{ duration: 0.2 }}
        className={cn([
          "rounded-full overflow-hidden flex items-center justify-center",
          isRecording || isProcessing ? "bg-neutral-900/90" : "bg-white/35",
        ])}
      >
        {isRecording && <Waveform levels={audioLevels} isLocked={phase === "locked"} />}
        {isProcessing && <Spinner />}
      </motion.div>
    </div>
  );
}

function Waveform({ levels, isLocked }: { levels: number[]; isLocked: boolean }) {
  const [heights, setHeights] = useState<number[]>(Array(10).fill(4));

  useEffect(() => {
    const frame = requestAnimationFrame(() => {
      setHeights((prev) =>
        prev.map((h, i) => {
          const bandIdx = Math.floor(i * 1.2);
          const bandAvg = levels[bandIdx] || 0;

          const centerBoost = 1 + (1 - Math.abs(i - 4.5) / 4.5) * 0.5;
          const target = 4 + Math.pow(bandAvg * centerBoost * 15, 0.7) * 16;

          const speed = target > h ? 0.6 : 0.25;
          return h + (target - h) * speed;
        }),
      );
    });
    return () => cancelAnimationFrame(frame);
  }, [levels]);

  return (
    <div className="flex items-center justify-center gap-0.5 h-full px-2">
      {heights.map((h, i) => (
        <div
          key={i}
          className={cn([
            "w-1 rounded-full transition-colors",
            isLocked ? "bg-orange-400" : "bg-white",
          ])}
          style={{ height: `${Math.min(h, 20)}px` }}
        />
      ))}
    </div>
  );
}

function Spinner() {
  return (
    <div className="flex items-center justify-center">
      <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
    </div>
  );
}
