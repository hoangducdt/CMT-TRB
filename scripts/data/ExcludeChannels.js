const EXCLUDE_CHANNELS = {
    X74: 
    {
        passSet: new Set(
`
NC
` .trim().split('\n').map(ch => ch.trim())
        )
    },
    SKL: {
        passSet: new Set(
`
NC
*_TDO
`.trim().split('\n').map(ch => ch.trim())
        )
    }
};