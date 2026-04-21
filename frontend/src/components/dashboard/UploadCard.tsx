import { useRef, useState, type DragEvent } from "react";
import type { WhatsResult } from "@/lib/types";

type Props = {
  onResult: (result: WhatsResult) => void;
  onLoadDemo?: () => void;
};

export function UploadCard({ onResult, onLoadDemo }: Props) {
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const endpoint = "http://localhost:3000/upload";

  const [file, setFile] = useState<File | null>(null);
  const [dragActive, setDragActive] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFile = (f: File) => {
    if (f && f.name.endsWith(".zip")) {
      setFile(f);
    } else {
      alert("Please upload a .zip file");
    }
  };

  const onDrop = (e: DragEvent) => {
    e.preventDefault();
    setDragActive(false);
    if (e.dataTransfer.files?.[0]) {
      handleFile(e.dataTransfer.files[0]);
    }
  };

  const isWhatsResult = (data: unknown): data is WhatsResult => {
    if (!data || typeof data !== "object") return false;
    const maybe = data as Partial<WhatsResult>;
    return (
      Array.isArray(maybe.user_results) &&
      (typeof maybe.most_active_user === "string" ||
        maybe.most_active_user === null)
    );
  };

  const onUpload = async () => {
    if (!file) return alert("No file selected");

    setLoading(true);
    setError(null);

    const formData = new FormData();
    formData.append("file", file);

    try {
      const response = await fetch(endpoint, {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        const body = await response.text();
        throw new Error(body || `Upload failed with status ${response.status}`);
      }

      const data: unknown = await response.json();
      if (!isWhatsResult(data)) {
        throw new Error(
          "Server response does not match expected analytics format",
        );
      }

      onResult(data);

      alert("Upload successful");
    } catch (err) {
      console.error(err);
      const message = err instanceof Error ? err.message : "Upload failed";
      setError(message);
      alert(message);
    } finally {
      setLoading(false);
    }
  };

  const loadSample = () => {
    if (onLoadDemo) {
      onLoadDemo();
      return;
    }
    alert("Hook your sample dataset here");
  };

  return (
    <div className="w-full max-w-2xl mx-auto bg-[#0f172a] text-white rounded-xl p-6 border border-white/10 shadow-lg">
      {/* Header */}
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">Upload WhatsApp export</h2>
      </div>

      {/* Instructions */}
      <div className="mb-4 rounded-lg border border-white/10 bg-black/20 p-4">
        <p className="text-sm font-semibold text-white">
          How to get your WhatsApp .zip
        </p>
        <ol className="mt-2 list-decimal space-y-2 pl-5 text-xs text-gray-300">
          <li>
            Android: open your WhatsApp chat, tap the 3 dots menu, then More
            if that option appears, and select Export chat.
          </li>
          <li>
            iOS: open the chat, go to Chat Details, scroll down, and tap Export
            Chat.
          </li>
          <li>Choose Export without media.</li>
          <li>
            Save the downloaded/extracted files wherever you want on your
            device.
          </li>
          <li>
            Back here, choose the exported .zip and upload it to get your
            insights.
          </li>
        </ol>
      </div>

      {/* Dropzone */}
      <div
        className={`border-2 border-dashed rounded-lg p-10 text-center transition ${
          dragActive ? "border-green-400 bg-green-400/10" : "border-white/20"
        }`}
        onDragOver={(e) => {
          e.preventDefault();
          setDragActive(true);
        }}
        onDragLeave={() => setDragActive(false)}
        onDrop={onDrop}
        onClick={() => fileInputRef.current?.click()}
      >
        <input
          ref={fileInputRef}
          type="file"
          accept=".zip"
          className="hidden"
          onChange={(e) => {
            if (e.target.files?.[0]) handleFile(e.target.files[0]);
          }}
        />

        <div className="text-green-400 text-2xl mb-2">⬆</div>

        <p className="font-medium">Drop .zip here or click to browse</p>

        <p className="text-xs text-gray-400 mt-1">
          WhatsApp chat export · max 50MB
        </p>

        {file && (
          <p className="mt-3 text-sm text-green-300">Selected: {file.name}</p>
        )}
      </div>

      {/* Actions */}
      {error && <p className="mt-4 text-sm text-red-300">{error}</p>}

      <div className="flex justify-between items-center mt-6">
        <button
          onClick={loadSample}
          className="text-sm text-gray-300 hover:text-white"
        >
          ✨ Load sample data
        </button>

        <button
          onClick={onUpload}
          disabled={!file || loading}
          className="bg-green-500 hover:bg-green-600 disabled:opacity-50 px-4 py-2 rounded text-sm font-medium"
        >
          {loading ? "Uploading..." : "Upload"}
        </button>
      </div>
    </div>
  );
}
