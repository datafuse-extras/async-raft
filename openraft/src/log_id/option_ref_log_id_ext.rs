use crate::log_id::ref_log_id::RefLogId;
use crate::type_config::alias::LogIdOf;
use crate::RaftTypeConfig;

pub(crate) trait OptionRefLogIdExt<C>
where C: RaftTypeConfig
{
    fn to_log_id(&self) -> Option<LogIdOf<C>>;
}

impl<C> OptionRefLogIdExt<C> for Option<RefLogId<'_, C>>
where C: RaftTypeConfig
{
    fn to_log_id(&self) -> Option<LogIdOf<C>> {
        self.as_ref().map(|r| r.to_log_id())
    }
}
