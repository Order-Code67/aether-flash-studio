import { create } from "zustand";
import type {
  ComPortInfo,
  DashboardSnapshot,
  DetectedDevice,
  UsbDeviceInfo,
} from "@/types/device";

interface DeviceStore {
  devices: DetectedDevice[];
  selectedDeviceId: string | null;
  comPorts: ComPortInfo[];
  usbDevices: UsbDeviceInfo[];
  dashboard: DashboardSnapshot | null;
  scanning: boolean;
  setDevices: (devices: DetectedDevice[]) => void;
  selectDevice: (id: string | null) => void;
  setComPorts: (ports: ComPortInfo[]) => void;
  setUsbDevices: (devices: UsbDeviceInfo[]) => void;
  setDashboard: (dashboard: DashboardSnapshot) => void;
  setScanning: (scanning: boolean) => void;
  selectedDevice: () => DetectedDevice | null;
}

export const useDeviceStore = create<DeviceStore>((set, get) => ({
  devices: [],
  selectedDeviceId: null,
  comPorts: [],
  usbDevices: [],
  dashboard: null,
  scanning: false,
  setDevices: (devices) => set({ devices }),
  selectDevice: (selectedDeviceId) => set({ selectedDeviceId }),
  setComPorts: (comPorts) => set({ comPorts }),
  setUsbDevices: (usbDevices) => set({ usbDevices }),
  setDashboard: (dashboard) => set({ dashboard }),
  setScanning: (scanning) => set({ scanning }),
  selectedDevice: () => {
    const { devices, selectedDeviceId } = get();
    return devices.find((d) => d.id === selectedDeviceId) ?? null;
  },
}));
