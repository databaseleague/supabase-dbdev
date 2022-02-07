import type { NextApiRequest, NextApiResponse } from 'next'
import { supabase } from '../../lib/supabaseClient'
import { definitions } from '../../types/database'
import { apiSuccess, apiNotFound, apiServerError } from '../../lib/helpers'

/**
 * TYPES
 *  Note that these aren't strictly accurate as the we haven't "Pick<>"ed the values
 */
export type AccountDetail = definitions['accounts'] & {
  organizations: definitions['organizations'][],
  username: string
}
const detailFields = `
    id, username: handle, 
    organizations (
        id, handle, display_name
    )
`

/**
 * API ENTRY
 */

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  try {
    // @TODO: supabase.setUser() using the Auth Bearer token? Or should we use some sort of access keys?
    return apiNotFound(res)
  } catch (error: unknown) {
    const e = error as Error
    return apiServerError(res, e.message)
  }
}

/**
 * API METHODS
 */

export async function account() {
  const userId = supabase.auth.user()?.id
  if (!userId) {
    return { error: 'Not logged in', data: null }
  }
  return supabase.from<AccountDetail>('accounts').select(detailFields).eq('id', userId).single()
}