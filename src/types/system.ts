export interface CpuInfo {
  name: string;
  manufacturer: string;
  cores: number;
  threads: number;
  base_speed_mhz: number;
  max_speed_mhz: number;
  usage_percent: number;
}

export interface RamModule {
  capacity_gb: number;
  speed_mhz: number;
  memory_type: string;
  manufacturer: string;
  slot: string;
}

export interface RamInfo {
  total_gb: number;
  used_gb: number;
  available_gb: number;
  usage_percent: number;
  modules: RamModule[];
}

export interface GpuInfo {
  name: string;
  vram_gb: number;
  driver_version: string;
}

export interface DiskInfo {
  model: string;
  size_gb: number;
  interface_type: string;
  partitions: DiskPartition[];
}

export interface DiskPartition {
  mount_point: string;
  fs_type: string;
  total_gb: number;
  used_gb: number;
  free_gb: number;
  usage_percent: number;
}

export interface MotherboardInfo {
  manufacturer: string;
  model: string;
}

export interface OsInfo {
  name: string;
  version: string;
  architecture: string;
  hostname: string;
  build_number: string;
}

export interface NetworkInfo {
  bytes_sent_mb: number;
  bytes_recv_mb: number;
  speed_up_kbs: number;
  speed_down_kbs: number;
}

export interface BatteryInfo {
  percent: number;
  plugged: boolean;
  time_left_minutes: number | null;
}

export interface SystemInfo {
  os: OsInfo;
  cpu: CpuInfo;
  ram: RamInfo;
  gpus: GpuInfo[];
  disks: DiskInfo[];
  motherboard: MotherboardInfo;
  network?: NetworkInfo;
  battery?: BatteryInfo | null;
}

export interface MonitorData {
  cpu_percent: number;
  cpu_per_core: number[];
  ram_percent: number;
  ram_used_gb: number;
  disk_percent: number;
  network_up_kbs: number;
  network_down_kbs: number;
  temperature_cpu: number | null;
  temperature_gpu: number | null;
  battery: BatteryInfo | null;
  top_processes: ProcessInfo[];
  alerts: SystemAlert[];
  timestamp: number;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_percent: number;
  memory_mb: number;
}

export interface SystemAlert {
  alert_type: "cpu" | "ram" | "disk" | "temperature";
  level: "warning" | "critical";
  message: string;
  timestamp: number;
}
