// Lifted from https://github.com/iamhosseindhv/notistack/issues/30#issuecomment-542863653

import { useSnackbar, VariantType, WithSnackbarProps } from 'notistack'
import React from 'react'

let useSnackbarRef: WithSnackbarProps
export const SnackbarUtilsConfigurator: React.FC = () => {
  useSnackbarRef = useSnackbar()
  return null
}

const SnackbarUtils = {
  success(msg: string) {
    this.toast(msg, 'success')
  },
  warning(msg: string) {
    this.toast(msg, 'warning')
  },
  info(msg: string) {
    this.toast(msg, 'info')
  },
  error(msg: string) {
    this.toast(msg, 'error')
  },
  toast(msg: string, variant: VariantType = 'default') {
    useSnackbarRef.enqueueSnackbar(msg, { variant })
  }
};

export default SnackbarUtils;