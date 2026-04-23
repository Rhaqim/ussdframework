'use client';

import React, { useEffect, useState } from 'react';

import { RouterOptions, Screens } from '@/api/route';
import Screen, { RouterOption } from '@/types/screen.type';

interface RouterOptionFormProps {
  screenName?: string;
  onSuccess?: () => void;
}

const RouterOptionForm = ({ screenName, onSuccess }: RouterOptionFormProps) => {
  const [screens, setScreens] = useState<Screen[]>([]);
  const [form, setForm] = useState<RouterOption>({
    screen_name: screenName ?? '',
    router_option: '',
    next_screen: '',
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    Screens.getAll()
      .then(setScreens)
      .catch(() => setScreens([]));
  }, []);

  const handleChange = (field: keyof RouterOption, value: string) => {
    setForm(prev => ({ ...prev, [field]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setSaving(true);
    try {
      await RouterOptions.create(form);
      onSuccess?.();
    } catch (err) {
      setError('Failed to save router option. Please try again.');
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

      {/* Expression */}
      <div>
        <label className={labelClass}>Condition Expression</label>
        <input
          className={inputClass}
          value={form.router_option}
          onChange={e => handleChange('router_option', e.target.value)}
          placeholder="{{data.status == 'success'}}"
          required
        />
        <p className="text-xs text-gray-400 mt-1">
          Use mustache-style expressions referencing session data, e.g.{' '}
          <code>{"{{key.field == 'value'}}"}</code>.
        </p>
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
        {saving ? 'Saving…' : 'Create Router Option'}
      </button>
    </form>
  );
};

export default RouterOptionForm;