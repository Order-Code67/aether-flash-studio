import { useEffect, useMemo, useState } from "react";
import Editor from "@monaco-editor/react";
import { api, type FirmwareArtifact } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { Button } from "@/components/ui/Button";
import { Tag } from "@/components/ui/Badge";

type Tab = "sources" | "advanced" | "custom" | "library" | "flash" | "history" | "settings";

function mapSubToTab(sub: string): Tab {
  if (sub === "library" || sub === "sources" || sub === "advanced" || sub === "custom") return sub;
  if (sub === "history" || sub === "settings") return sub;
  return "sources";
}

export function FirmwareStudioPage() {
  const t = useT();
  const pushLog = useAppStore((s) => s.pushLog);
  const subSection = useAppStore((s) => s.subSection);
  const setSubSection = useAppStore((s) => s.setSubSection);
  const [tab, setTab] = useState<Tab>(() => mapSubToTab(subSection));
  const [releases, setReleases] = useState<Array<Record<string, unknown>>>([]);
  const [locals, setLocals] = useState<Array<Record<string, unknown>>>([]);
  const [selectedRef, setSelectedRef] = useState("");
  const [workspaceRoot, setWorkspaceRoot] = useState("");
  const [tree, setTree] = useState<Array<{ path: string; name: string; isDir: boolean }>>([]);
  const [filePath, setFilePath] = useState("");
  const [fileContent, setFileContent] = useState("");
  const [language, setLanguage] = useState("plaintext");
  const [search, setSearch] = useState("");
  const [searchHits, setSearchHits] = useState<string[]>([]);
  const [diff, setDiff] = useState("");
  const [buildLog, setBuildLog] = useState("");
  const [artifacts, setArtifacts] = useState<FirmwareArtifact[]>([]);
  const [custom, setCustom] = useState({
    manufacturer: "ExpressLRS",
    deviceType: "rx",
    platform: "ESP8285",
    model: "Generic RX",
    elrsVersion: "",
    target: "Unified_ESP8285_2400_RX",
  });

  async function refreshLibrary() {
    setArtifacts(await api.listFirmwareArtifacts());
  }

  async function refreshSources() {
    const [r, l] = await Promise.all([api.listElrsReleases(), api.listElrsLocalSources()]);
    setReleases(r as Array<Record<string, unknown>>);
    setLocals(l as Array<Record<string, unknown>>);
  }

  useEffect(() => {
    void refreshSources().catch((e) => pushLog("ERROR", String(e)));
    void refreshLibrary().catch((e) => pushLog("ERROR", String(e)));
  }, []);

  useEffect(() => {
    setTab(mapSubToTab(subSection));
  }, [subSection]);

  const fileNodes = useMemo(() => tree.filter((n) => !n.isDir).slice(0, 800), [tree]);

  async function openWorkspace(root: string) {
    setWorkspaceRoot(root);
    const nodes = await api.workspaceListTree(root);
    setTree(nodes as Array<{ path: string; name: string; isDir: boolean }>);
    pushLog("INFO", t("fw.workspaceOpened", { root }));
  }

  const tabs: Array<[Tab, string]> = [
    ["library", t("fw.tab.library")],
    ["sources", t("fw.tab.sources")],
    ["advanced", t("fw.tab.advanced")],
    ["custom", t("fw.tab.custom")],
    ["history", t("nav.sub.buildHistory")],
    ["settings", t("nav.sub.buildSettings")],
  ];

  function selectTab(id: Tab) {
    setTab(id);
    setSubSection(id === "library" ? "library" : id);
  }

  const customFields: Array<[keyof typeof custom, string]> = [
    ["manufacturer", t("fw.manufacturer")],
    ["deviceType", t("fw.deviceType")],
    ["platform", t("fw.platform")],
    ["model", t("fw.model")],
    ["elrsVersion", t("fw.elrsVersion")],
    ["target", t("fw.target")],
  ];

  return (
    <div className="flex h-full flex-col gap-3 overflow-hidden p-4">
      <div className="flex flex-wrap items-end justify-between gap-2">
        <div>
          <h2 className="text-lg font-semibold">{t("fw.title")}</h2>
          <p className="text-sm text-graphite-400">{t("fw.subtitle")}</p>
        </div>
        <div className="flex flex-wrap gap-1">
          {tabs.map(([id, label]) => (
            <Button
              key={id}
              size="sm"
              variant={tab === id ? "primary" : "secondary"}
              onClick={() => selectTab(id)}
            >
              {label}
            </Button>
          ))}
        </div>
      </div>

      {tab === "sources" && (
        <div className="grid min-h-0 flex-1 gap-3 overflow-auto xl:grid-cols-2">
          <section className="panel-inset space-y-2 rounded-lg p-3">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-semibold">{t("fw.officialReleases")}</h3>
              <Button size="sm" onClick={() => void refreshSources()}>
                {t("fw.refresh")}
              </Button>
            </div>
            <p className="text-[11px] text-graphite-500">{t("fw.sourceOnly")}</p>
            <div className="max-h-72 space-y-1 overflow-auto text-xs">
              {releases.map((r) => (
                <button
                  key={String(r.tagName)}
                  className={`block w-full rounded border px-2 py-1 text-left ${
                    selectedRef === String(r.tagName)
                      ? "border-accent-teal/50 bg-accent-teal/10"
                      : "border-graphite-700"
                  }`}
                  onClick={() => setSelectedRef(String(r.tagName))}
                >
                  <div className="font-mono">{String(r.tagName)}</div>
                  <div className="text-graphite-500">
                    {r.prerelease ? t("fw.prerelease") : t("fw.release")} ·{" "}
                    {String(r.publishedAt || "")}
                  </div>
                </button>
              ))}
            </div>
            <div className="flex gap-2">
              <input
                className="flex-1 rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono text-xs"
                placeholder={t("fw.tagOrBranch")}
                value={selectedRef}
                onChange={(e) => setSelectedRef(e.target.value)}
              />
              <Button
                variant="primary"
                onClick={async () => {
                  try {
                    const rec = await api.fetchElrsSource({
                      versionOrRef: selectedRef,
                      kind: "release",
                    });
                    pushLog(
                      "INFO",
                      t("fw.fetched", { ref: rec.versionOrRef, sha: rec.commitSha }),
                    );
                    setWorkspaceRoot(rec.localPath);
                    await refreshSources();
                  } catch (e) {
                    pushLog("ERROR", String(e));
                  }
                }}
              >
                {t("fw.download")}
              </Button>
            </div>
          </section>
          <section className="panel-inset space-y-2 rounded-lg p-3">
            <h3 className="text-sm font-semibold">{t("fw.localCopies")}</h3>
            <div className="max-h-96 space-y-2 overflow-auto text-xs">
              {locals.length === 0 ? (
                <p className="text-graphite-500">{t("fw.noLocal")}</p>
              ) : (
                locals.map((l) => (
                  <div key={String(l.id)} className="rounded border border-graphite-700 p-2">
                    <div className="font-mono">{String(l.versionOrRef)}</div>
                    <div className="truncate text-graphite-500">{String(l.localPath)}</div>
                    <div className="mt-1 flex gap-2">
                      <Tag tone={l.integrityOk ? "teal" : "warn"}>
                        {l.integrityOk ? t("fw.integrityOk") : t("fw.check")}
                      </Tag>
                      <Button size="sm" onClick={() => void openWorkspace(String(l.localPath))}>
                        {t("fw.openAdvanced")}
                      </Button>
                    </div>
                  </div>
                ))
              )}
            </div>
          </section>
        </div>
      )}

      {tab === "advanced" && (
        <div className="grid min-h-0 flex-1 grid-rows-[auto_1fr_140px] gap-2 overflow-hidden">
          <div className="flex flex-wrap gap-2">
            <input
              className="min-w-[280px] flex-1 rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono text-xs"
              placeholder={t("fw.workspaceRoot")}
              value={workspaceRoot}
              onChange={(e) => setWorkspaceRoot(e.target.value)}
            />
            <Button
              size="sm"
              onClick={() => void openWorkspace(workspaceRoot).catch((e) => pushLog("ERROR", String(e)))}
            >
              {t("fw.loadTree")}
            </Button>
            <input
              className="rounded border border-graphite-600 bg-graphite-950 px-2 py-2 text-xs"
              placeholder={t("fw.search")}
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
            <Button
              size="sm"
              onClick={async () => {
                try {
                  setSearchHits(await api.workspaceSearch(workspaceRoot, search));
                } catch (e) {
                  pushLog("ERROR", String(e));
                }
              }}
            >
              {t("fw.find")}
            </Button>
          </div>
          <div className="grid min-h-0 grid-cols-[220px_1fr_200px] gap-2 overflow-hidden">
            <div className="panel-inset overflow-auto rounded-lg p-2 text-[11px]">
              {fileNodes.map((n) => (
                <button
                  key={n.path}
                  className={`block w-full truncate rounded px-1 py-0.5 text-left hover:bg-graphite-800 ${
                    filePath === n.path ? "bg-accent-teal/15 text-accent-teal" : ""
                  }`}
                  onClick={async () => {
                    try {
                      const f = await api.workspaceReadFile(workspaceRoot, n.path);
                      setFilePath(f.path);
                      setFileContent(f.content);
                      setLanguage(f.language);
                      setDiff(await api.workspaceDiff(workspaceRoot, n.path));
                    } catch (e) {
                      pushLog("ERROR", String(e));
                    }
                  }}
                >
                  {n.path}
                </button>
              ))}
              {searchHits.length > 0 && (
                <div className="mt-2 border-t border-graphite-700 pt-2">
                  <div className="mb-1 text-graphite-500">{t("fw.hits")}</div>
                  {searchHits.map((h) => (
                    <div key={h} className="truncate text-accent-blue">
                      {h}
                    </div>
                  ))}
                </div>
              )}
            </div>
            <div className="overflow-hidden rounded-lg border border-graphite-700">
              <Editor
                height="100%"
                theme="vs-dark"
                language={language}
                value={fileContent}
                onChange={(v) => setFileContent(v ?? "")}
                options={{ minimap: { enabled: false }, fontSize: 12 }}
              />
            </div>
            <div className="panel-inset space-y-2 overflow-auto rounded-lg p-2 text-xs">
              <div className="font-semibold">{t("fw.properties")}</div>
              <div className="font-mono text-[10px] text-graphite-400">{filePath || "—"}</div>
              <div>
                {t("fw.language")}: {language}
              </div>
              <Button
                size="sm"
                className="w-full"
                onClick={async () => {
                  try {
                    await api.workspaceWriteFile(workspaceRoot, filePath, fileContent);
                    pushLog("INFO", t("fw.saved", { path: filePath }));
                    setDiff(await api.workspaceDiff(workspaceRoot, filePath));
                  } catch (e) {
                    pushLog("ERROR", String(e));
                  }
                }}
              >
                {t("fw.save")}
              </Button>
              <Button
                size="sm"
                className="w-full"
                onClick={async () => {
                  try {
                    await api.workspaceRevertFile(workspaceRoot, filePath);
                    const f = await api.workspaceReadFile(workspaceRoot, filePath);
                    setFileContent(f.content);
                    pushLog("INFO", t("fw.reverted", { path: filePath }));
                  } catch (e) {
                    pushLog("ERROR", String(e));
                  }
                }}
              >
                {t("fw.revert")}
              </Button>
              <pre className="whitespace-pre-wrap text-[10px] text-graphite-400">{diff}</pre>
            </div>
          </div>
          <div className="panel-inset overflow-auto rounded-lg p-2 font-mono text-[11px] text-graphite-300">
            {buildLog || t("fw.terminal")}
          </div>
        </div>
      )}

      {tab === "custom" && (
        <div className="grid max-w-3xl gap-3 overflow-auto">
          <section className="panel-inset space-y-2 rounded-lg p-4 text-sm">
            <h3 className="font-semibold">{t("fw.customTitle")}</h3>
            {customFields.map(([key, label]) => (
              <label key={key} className="block text-xs">
                <span className="text-graphite-400">{label}</span>
                <input
                  className="mt-1 w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2"
                  value={custom[key]}
                  onChange={(e) => setCustom({ ...custom, [key]: e.target.value })}
                />
              </label>
            ))}
            <label className="block text-xs">
              <span className="text-graphite-400">{t("fw.sourcePath")}</span>
              <input
                className="mt-1 w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono"
                value={workspaceRoot}
                onChange={(e) => setWorkspaceRoot(e.target.value)}
              />
            </label>
            <div className="flex gap-2">
              <Button
                variant="primary"
                onClick={async () => {
                  try {
                    setBuildLog(t("fw.buildStart"));
                    const result = await api.runCustomFirmwareBuild({
                      manufacturer: custom.manufacturer,
                      deviceType: custom.deviceType,
                      platform: custom.platform,
                      model: custom.model,
                      elrsVersion: custom.elrsVersion || selectedRef,
                      sourcePath: workspaceRoot,
                      target: custom.target,
                      config: { advanced: false },
                    });
                    setBuildLog((prev) => prev + JSON.stringify(result, null, 2));
                    await refreshLibrary();
                    pushLog("INFO", t("fw.buildDone"));
                  } catch (e) {
                    setBuildLog((prev) => prev + `\nERROR: ${String(e)}`);
                    pushLog("ERROR", String(e));
                  }
                }}
              >
                {t("fw.compile")}
              </Button>
              <Button size="sm" onClick={() => setTab("advanced")}>
                {t("fw.advancedMode")}
              </Button>
            </div>
            <pre className="max-h-48 overflow-auto rounded border border-graphite-700 bg-graphite-950 p-2 font-mono text-[11px]">
              {buildLog || t("fw.buildLogEmpty")}
            </pre>
          </section>
        </div>
      )}

      {(tab === "history" || tab === "settings") && (
        <section className="panel-inset max-w-2xl overflow-auto p-4 text-sm text-graphite-400">
          <h3 className="mb-2 text-sm font-semibold text-graphite-100">
            {tab === "history" ? t("nav.sub.buildHistory") : t("nav.sub.buildSettings")}
          </h3>
          <p>{tab === "history" ? t("fw.historyHelp") : t("fw.settingsHelp")}</p>
          <pre className="mt-3 overflow-auto font-mono text-[11px]">{buildLog || t("fw.buildLogEmpty")}</pre>
        </section>
      )}

      {tab === "library" && (
        <section className="panel-inset overflow-auto rounded-lg p-4">
          <h3 className="mb-2 text-sm font-semibold">{t("fw.library")}</h3>
          <div className="space-y-1 text-xs">
            {artifacts.map((a) => (
              <button
                key={a.id}
                className="flex w-full justify-between border-b border-graphite-800 py-1 text-left"
                onClick={() => undefined}
              >

                <span>
                  {a.version} · {a.channel} {a.verified ? "✓" : ""}
                </span>
                <span className="font-mono text-graphite-500">{a.sha256.slice(0, 12)}</span>
              </button>
            ))}
          </div>
        </section>
      )}

    </div>
  );
}
