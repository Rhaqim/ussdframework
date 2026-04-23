'use client';

import React, { useEffect, useState } from 'react';

import { MenuItems, Screens } from '@/api/route';
import Screen, { MenuItem } from '@/types/screen.type';

interface MenuItemFormProps {
  screenName?: string;
  onSuccess?: () => void;
}

const MenuItemForm = ({ screenName, onSuccess }: MenuItemFormProps) => {
  const [screens, setScreens] = useState<Screen[]>([]);
  const [form, setForm] = useState<MenuItem>({
    screen_name: screenName ?? '',
    name: '',
    option: '',
    display_name: '',
    next_screen: '',
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    Screens.getAll()
      .then(setScreens)
      .catch(() => setScreens([]));
  }, []);

  const handleChange = (field: keyof MenuItem, value: string) => {
    setForm(prev => ({ ...prev, [field]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSaving(true);
    try {
      await MenuItems.create(form);
      onSuccess?.();
    } catch (err) {
      setError('Failed to save menu item. Please try again.');
      console.error(err);
    } finally {
      setSaving(false);
    }
  };

  const inputClass =
    'mt-1 w-full border border-gray-300 rounded px-3 py-2 text-sm text-gray-800 focus:outline-none focus:border-blue-400';
  const labelClass = 'block text-sm font-medium text-gray-200';

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {/* Screen Name */}
      <div>
        <label className={labelClass}>Screen</label>
        <select
          className={inputClass}
          value={form.screen_name}
          onChange={e => handleChange('screen_name', e.target.value)}
          required
          disabled={!!screenName}
        >
          <option value="">Select a screen…</option>
          {screens.map(s => (
            <option key={s.name} value={s.name}>
              {s.name}
            </option>
          ))}
        </select>
      </div>

      {/* Internal name */}
      <div>
        <label className={labelClass}>Internal Name</label>
        <input
          className={inputClass}
          value={form.name}
          onChange={e => handleChange('name', e.target.value)}
          placeholder="e.g. buy_airtime"
          required
        />
      </div>

      {/* Option number */}
      <div>
        <label className={labelClass}>Option Number</label>
        <input
          className={inputClass}
          type="number"
          min={1}
          value={form.option}
          onChange={e => handleChange('option', e.target.value)}
          placeholder="1"
          required
        />
      </div>

      {/* Display name */}
      <div>
        <label className={labelClass}>Display Name</label>
        <input
          className={inputClass}
          value={form.display_name}
          onChange={e => handleChange('display_name', e.target.value)}
          placeholder="Buy Airtime"
          required
        />
      </div>

      {/* Next screen */}
      <div>
        <label className={labelClass}>Next Screen</label>
        <select
          className={inputClass}
          value={form.next_screen}
          onChange={e => handleChange('next_screen', e.target.value)}
          required
        >
          <option value="">Select next screen…</option>
          {screens.map(s => (
            <option key={s.name} value={s.name}>
              {s.name}
            </option>
          ))}
        </select>
      </div>

      {error && <p className="text-red-400 text-sm">{error}</p>}

      <button
        type="submit"
        disabled={saving}
        className="bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white text-sm font-medium rounded px-4 py-2 w-full transition-colors"
      >
        {saving ? 'Saving…' : 'Create Menu Item'}
      </button>
    </form>
  );
};

export default MenuItemForm;