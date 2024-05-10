import React from 'react'

import Form from '@/components/UI/Form'
import Screen, { RouterOption, MenuItem } from '@/types/screen.type'

const ScreenForm = () => {
  const fields = [
    {
      name: 'name',
      label: 'Name',
      type: 'text',
    },
    {
      name: 'text',
      label: 'Text',
      type: 'text',
    },
    {
      name: 'screen_type',
      label: 'Screen Type',
      type: 'dropdown',
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

  const handleSubmit = (data: Screen) => {
    console.log(data)
  }

  return <Form fields={fields} onSubmit={handleSubmit} />
}

export default ScreenForm