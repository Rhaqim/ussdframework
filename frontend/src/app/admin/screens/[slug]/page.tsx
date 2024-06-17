'use client';

import React from 'react'

import { Screens } from '@/api/route'
import ReusableForm from '@/components/UI/RefilledForm'

const Screen = () => {
  const fields = [
    {
      name: 'name',
      label: 'Name',
      type: 'text',
      value: 'Initial',
    },
    {
      name: 'text',
      label: 'Text',
      type: 'text',
      value: 'Initial',
    },
    {
      name: 'screen_type',
      label: 'Screen Type',
      type: 'dropdown',
      value: 'initial',
      options: [
        { value: 'initial', label: 'Initial' },
        { value: 'quit', label: 'Quit' },
        { value: 'function', label: 'Function' },
        { value: 'router', label: 'Router' },
        { value: 'menu', label: 'Menu' },
        { value: 'input', label: 'Input' },
      ],
    },
    {
      name: 'default_next_screen',
      label: 'Default Next Screen',
      value: 'Initial',
      type: 'text',
    },
    {
      name: 'service_code',
      label: 'Service Code',
      type: 'text',
    },
    {
      name: 'input_identifier',
      label: 'Input Identifier',
      type: 'text',
    },
    {
      name: 'input_type',
      label: 'Input Type',
      type: 'text',
    },
  ]


  return (
    <div>
      <ReusableForm fields={fields} onSubmit={(data) => Screens.create(data)} />
    </div>
  )
}

export default Screen