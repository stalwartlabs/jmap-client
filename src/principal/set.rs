use std::collections::HashMap;

use crate::{core::set::SetObject, Get, Set};

use super::{Principal, Type, ACL, DKIM};

impl Principal<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into().into();
        self
    }

    pub fn description(&mut self, description: Option<impl Into<String>>) -> &mut Self {
        self.description = description.map(|s| s.into());
        self
    }

    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        self.email = email.into().into();
        self
    }

    pub fn secret(&mut self, secret: impl Into<String>) -> &mut Self {
        self.secret = secret.into().into();
        self
    }

    pub fn timezone(&mut self, timezone: Option<impl Into<String>>) -> &mut Self {
        self.timezone = timezone.map(|s| s.into());
        self
    }

    pub fn picture(&mut self, picture: Option<impl Into<String>>) -> &mut Self {
        self.picture = picture.map(|s| s.into());
        self
    }

    pub fn quota(&mut self, quota: Option<u32>) -> &mut Self {
        self.quota = quota;
        self
    }

    pub fn ptype(&mut self, ptype: Type) -> &mut Self {
        self.ptype = ptype.into();
        self
    }

    pub fn dkim(&mut self, dkim: DKIM) -> &mut Self {
        self.dkim = dkim.into();
        self
    }

    pub fn acl(&mut self, acl: Option<HashMap<String, Vec<ACL>>>) -> &mut Self {
        self.acl = acl;
        self
    }

    pub fn aliases<T, U>(&mut self, aliases: Option<T>) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.aliases = aliases.map(|l| l.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn capabilities<T, U>(&mut self, capabilities: Option<T>) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.capabilities = capabilities.map(|l| l.into_iter().map(|v| v.into()).collect());
        self
    }

    pub fn members<T, U>(&mut self, members: Option<T>) -> &mut Self
    where
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        self.members = members.map(|l| l.into_iter().map(|v| v.into()).collect());
        self
    }
}

impl SetObject for Principal<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        Principal {
            _create_id,
            _state: Default::default(),
            id: None,
            ptype: None,
            name: "".to_string().into(),
            description: "".to_string().into(),
            email: "".to_string().into(),
            timezone: "".to_string().into(),
            capabilities: Vec::with_capacity(0).into(),
            aliases: Vec::with_capacity(0).into(),
            secret: "".to_string().into(),
            dkim: None,
            quota: None,
            picture: "".to_string().into(),
            members: Vec::with_capacity(0).into(),
            acl: HashMap::with_capacity(0).into(),
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for Principal<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}