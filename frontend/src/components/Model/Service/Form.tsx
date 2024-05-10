'use client'

import React from 'react'

import Form from '@/components/UI/Form'
import Service from '@/types/service.type'

const ServiceForm = () => {
  const fields = [
    {
      name: 'name',
      label: 'Name',
      type: 'text',
    },
    {
      name: 'function_name',
      label: 'Function Name',
      type: 'text',
    },
    {
      name: 'function_url',
      label: 'Function URL',
      type: 'text',
    },
    {
      name: 'data_key',
      label: 'Data Key',
      type: 'text',
    },
    {
      name: 'service_code',
      label: 'Service Code',
      type: 'text',
    },
  ]

  const handleSubmit = (data: Service) => {
    console.log(data)
  }

  return <Form fields={fields} onSubmit={handleSubmit} />
}

export default ServiceForm