"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

/* ─── Inline SVG icons ─────────────────────────────────────────── */
const DashboardIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <rect x="3" y="3" width="7" height="7" rx="1.5" /><rect x="14" y="3" width="7" height="7" rx="1.5" />
    <rect x="3" y="14" width="7" height="7" rx="1.5" /><rect x="14" y="14" width="7" height="7" rx="1.5" />
  </svg>
);
const NodesIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <circle cx="4" cy="12" r="2" /><circle cx="20" cy="5" r="2" /><circle cx="20" cy="19" r="2" />
    <path d="M6 12h5l5-5.5M11 12l5 5.5" />
  </svg>
);
const ScreensIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <rect x="2" y="3" width="20" height="14" rx="2" /><path d="M8 21h8M12 17v4" />
  </svg>
);
const MenuItemsIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" />
  </svg>
);
const RouterIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <path d="M5 9l7 3-7 3M12 12h8" /><path d="M12 6l4-3 4 3M12 18l4 3 4-3" />
  </svg>
);
const ServicesIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <circle cx="12" cy="12" r="3" />
    <path d="M12 2v3M12 19v3M4.22 4.22l2.12 2.12M17.66 17.66l2.12 2.12M2 12h3M19 12h3M4.22 19.78l2.12-2.12M17.66 6.34l2.12-2.12" />
  </svg>
);
const ExportIcon = () => (
  <svg className="w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
    <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
  </svg>
);

/* ─── Nav items ─────────────────────────────────────────────────── */
const navItems = [
  { href: "/admin",               label: "Dashboard",      icon: DashboardIcon,  exact: true  },
  { href: "/admin/menu_nodes",    label: "Visual Editor",  icon: NodesIcon,      exact: false },
  { href: "/admin/screens",       label: "Screens",        icon: ScreensIcon,    exact: false },
  { href: "/admin/menu_items",    label: "Menu Items",     icon: MenuItemsIcon,  exact: false },
  { href: "/admin/router_options",label: "Router Options", icon: RouterIcon,     exact: false },
  { href: "/admin/services",      label: "Services",       icon: ServicesIcon,   exact: false },
  { href: "/admin/export",        label: "Import / Export",icon: ExportIcon,     exact: false },
];

/* ─── Component ─────────────────────────────────────────────────── */
export default function SideBar() {
  const pathname = usePathname();

  return (
    <aside className="fixed left-0 top-0 h-full w-60 bg-slate-900 border-r border-slate-800 flex flex-col z-40">
      {/* Brand */}
      <div className="flex items-center gap-3 px-5 py-[18px] border-b border-slate-800">
        <div className="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center shrink-0">
          <svg className="w-4 h-4 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2.5}>
            <path d="M9 3H5a2 2 0 00-2 2v4m6-6h10a2 2 0 012 2v4M9 3v18m0 0h10a2 2 0 002-2V9M9 21H5a2 2 0 01-2-2V9m0 0h18" />
          </svg>
        </div>
        <div>
          <div className="text-slate-100 font-semibold text-sm leading-tight">USSD Studio</div>
          <div className="text-slate-500 text-xs">Admin Portal</div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-3 py-4 space-y-0.5 overflow-y-auto">
        <div className="text-[10px] font-semibold text-slate-600 uppercase tracking-widest px-3 mb-2">
          Navigation
        </div>
        {navItems.map(item => {
          const isActive = item.exact
            ? pathname === item.href
            : pathname.startsWith(item.href);
          const Icon = item.icon;
          return (
            <Link
              key={item.href}
              href={item.href}
              className={`flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition-all ${
                isActive
                  ? "bg-indigo-600/15 text-indigo-400 border border-indigo-600/20 font-medium"
                  : "text-slate-400 hover:text-slate-100 hover:bg-slate-800 border border-transparent"
              }`}
            >
              <Icon />
              <span>{item.label}</span>
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="px-5 py-4 border-t border-slate-800">
        <div className="text-[11px] text-slate-600">USSD Framework · v0.1.0</div>
      </div>
    </aside>
  );
}

