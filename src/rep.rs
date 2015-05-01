#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SearchResult {
  pub description: String,
  pub is_official: bool,
  pub is_trusted: bool,
  pub name: String,
  pub star_count: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Image {
  pub Created: u64,
  pub Id: String,
  pub ParentId: String,
  //pub Labels: ???,
  pub RepoTags: Vec<String>,
  pub RepoDigests: Option<Vec<String>>,
  pub Size: u64,
  pub VirtualSize: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct ImageDetails {
  pub Architecture: String,
  pub Author: String,
  pub Comment: String,
  pub Config: Config,
  pub Created: String,
  pub DockerVersion: String,
  pub Id: String,
  pub Os: String,
  pub Parent: String,
  pub Size: u64,
  pub VirtualSize: u64  
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Container {
  pub Created: u64,
  pub Command: String,
  pub Id: String,
  pub Image: String,
  //pub Labels: ???,
  pub Names: Vec<String>,
  pub Ports: Vec<Port>,
  pub Status: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct ContainerDetails {
  pub AppArmorProfile: String,
  pub Args: Vec<String>,
  pub Config: Config,
  pub Created: String,
  pub Driver: String,
  pub ExecDriver: String,
  //pub ExecIDs: ??
  pub HostConfig: HostConfig
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct HostConfig {
  pub CgroupParent: Option<String>,
  pub ContainerIDFile: String,
  pub CpuShares: Option<u64>,
  pub CpusetCpus: Option<String>,
  pub Memory: Option<u64>,
  pub MemorySwap: Option<u64>,
  pub NetworkMode: String,
  pub PidMode: Option<String>,
  //pub PortBindings: ???
  pub Privileged: bool,
  pub PublishAllPorts: bool,
  pub ReadonlyRootfs: Option<bool>
  //pub RestartPolicy: ???
  //pub SecurityOpt: Option<???>,
  //pub Ulimits: Option<???>
 // pub VolumesFrom: Option<??/>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Config {
  AttachStderr: bool,
  AttachStdin: bool,
  AttachStdout: bool,
  Cmd: Vec<String>,
  CpuShares: u64,
  Cpuset: String,
  Domainname: String,
  Entrypoint: Vec<String>,
  Env: Vec<String>,
  //ExposedPorts
  Hostname: String,
  Image: String,
  //Labels:???
  MacAddress: String,
  Memory: u64,
  MemorySwap: u64,
  NetworkDisabled: bool,
  //OnBuild: Option<String>,
  OpenStdin: bool,
  //PortSpecs: ???,
  StdinOnce: bool,
  Tty: bool,
  User: String,
  //Volumes: ??,
  WorkingDir: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Port {
  pub IP: Option<String>,
  pub PrivatePort: u64,
  pub PublicPort: Option<u64>,
  pub Type: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Stats {
  pub read: String,
  pub network: Network,
  pub memory_stats: MemoryStats,
  pub cpu_stats: CpuStats
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Network {
  pub rx_dropped: u64,
  pub rx_bytes: u64,
  pub rx_errors: u64,
  pub tx_packets: u64,
  pub tx_dropped: u64,
  pub rx_packets: u64,
  pub tx_errors: u64,
  pub tx_bytes: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MemoryStats {
  pub max_usage: u64,
  pub usage: u64,
  pub failcnt: u64,
  pub limit: u64,
  pub stats: MemoryStat
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct MemoryStat {
  pub total_pgmajfault: u64,
  pub cache: u64,
  pub mapped_file: u64,
  pub total_inactive_file: u64,
  pub pgpgout: u64,
  pub rss: u64,
  pub total_mapped_file: u64,
  pub writeback: u64,
  pub unevictable: u64,
  pub pgpgin: u64,
  pub total_unevictable: u64,
  pub pgmajfault: u64,
  pub total_rss: u64,
  pub total_rss_huge: u64,
  pub total_writeback: u64,
  pub total_inactive_anon: u64,
  pub rss_huge: u64,
  pub hierarchical_memory_limit: u64,
  pub hierarchical_memsw_limit: u64,
  pub total_pgfault: u64,
  pub total_active_file: u64,
  pub active_anon: u64,
  pub total_active_anon: u64,
  pub total_pgpgout: u64,
  pub total_cache: u64,
  pub inactive_anon: u64,
  pub active_file: u64,
  pub pgfault: u64,
  pub inactive_file: u64,
  pub total_pgpgin: u64,
  pub swap: u64,
  pub total_swap: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CpuStats {
  pub cpu_usage: CpuUsage,
  pub system_cpu_usage: u64,
  pub throttling_data: ThrottlingData
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CpuUsage {
  pub percpu_usage: Vec<u64>,
  pub usage_in_usermode: u64,
  pub total_usage: u64,
  pub usage_in_kernelmode: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct ThrottlingData {
  pub periods: u64,
  pub throttled_periods: u64,
  pub throttled_time: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BlkioStats {
  pub io_service_bytes_recursive: Vec<BlkioStat>,
  pub io_serviced_recursive: Vec<BlkioStat>,
  pub io_queue_recursive: Vec<BlkioStat>,
  pub io_service_time_recursive: Vec<BlkioStat>,
  pub io_wait_time_recursive: Vec<BlkioStat>,
  pub io_merged_recursive: Vec<BlkioStat>,
  pub io_time_recursive: Vec<BlkioStat>,
  pub sectors_recursive: Vec<BlkioStat>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BlkioStat {
  pub major: u64,
  pub minor: u64,
  pub op: String,
  pub value: u64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Change {
  pub Kind: u64,
  pub Path: String
}


#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Top {
  pub Titles: Vec<String>,
  pub Processes: Vec<Vec<String>>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Version {
  pub ApiVersion: String,
  pub Version: String,
  pub GitCommit: String,
  pub GoVersion: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Info {
  pub Containers: u64,
  pub Images: u64,
  pub Driver: String,
  pub DockerRootDir: String,
  pub DriverStatus: Vec<Vec<String>>,
  pub ExecutionDriver: String,
  pub ID: String,
  pub KernelVersion: String,
  //pub Labels: Option<???>,
  pub MemTotal: u64,
  pub MemoryLimit: u64,
  pub NCPU: u64,
  pub NEventsListener: u64,
  pub NGoroutines: u64,
  pub Name: String,
  pub OperatingSystem: String,
  //pub RegistryConfig:???
  pub SwapLimit: u64,
  pub SystemTime: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
#[allow(non_snake_case)]
pub struct ContainerCreateInfo {
  pub Id: String,
  pub Warnings: Option<Vec<String>>
}