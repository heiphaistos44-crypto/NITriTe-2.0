import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke, InvokeTimeoutError } from "@/utils/invoke";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke as tauriInvoke } from "@tauri-apps/api/core";
const mockInvoke = vi.mocked(tauriInvoke);

describe("invoke() — timeout wrapper", () => {
  beforeEach(() => { mockInvoke.mockReset(); });

  it("retourne le résultat si la commande répond dans le délai", async () => {
    mockInvoke.mockResolvedValue({ name: "Windows 11" });
    const result = await invoke<{ name: string }>("get_system_info");
    expect(result.name).toBe("Windows 11");
  });

  it("lance InvokeTimeoutError si la commande dépasse le timeout", async () => {
    // Simule une commande qui ne répond jamais
    mockInvoke.mockImplementation(() => new Promise(() => {}));
    await expect(invoke("get_system_info", undefined, 10)).rejects.toThrow(InvokeTimeoutError);
  });

  it("InvokeTimeoutError contient le nom de la commande", async () => {
    mockInvoke.mockImplementation(() => new Promise(() => {}));
    try {
      await invoke("get_wmi_data", undefined, 10);
    } catch (e) {
      expect(e).toBeInstanceOf(InvokeTimeoutError);
      expect((e as InvokeTimeoutError).message).toContain("get_wmi_data");
    }
  });

  it("passe les arguments correctement à tauri invoke", async () => {
    mockInvoke.mockResolvedValue([]);
    await invoke("get_folder_sizes", { path: "C:\\" });
    expect(mockInvoke).toHaveBeenCalledWith("get_folder_sizes", { path: "C:\\" });
  });
});
