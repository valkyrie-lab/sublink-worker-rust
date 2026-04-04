include $(TOPDIR)/rules.mk

PKG_NAME:=sublink
PKG_VERSION:=1.0.0
PKG_RELEASE:=1
PKG_LICENSE:=MIT

include $(INCLUDE_DIR)/package.mk

define Package/sublink
  SECTION:=net
  CATEGORY:=Network
  SUBMENU:=Web Servers/Proxies
  TITLE:=Sublink Worker
  DESCRIPTION:=A lightweight proxy worker service for sublinks
  DEPENDS:=+libpthread +TARGET_OPTIMIZATION
  MAINTAINER:=your_name
endef

define Package/sublink/install
	$(INSTALL_DIR) $(1)/bin
	$(INSTALL_BIN) $(CURDIR)/files/sublink $(1)/bin/sublink

	$(INSTALL_DIR) $(1)/etc/init.d
	$(INSTALL_BIN) $(CURDIR)/files/sublink.init $(1)/etc/init.d/sublink
	chmod 755 $(1)/etc/init.d/sublink

	$(INSTALL_DIR) $(1)/etc/sublink
	$(INSTALL_DATA) $(CURDIR)/files/config.toml.example $(1)/etc/sublink/config.toml
endef

define Package/sublink/postinst
#!/bin/sh
[ -n "$${IPKG_INSTROOT}" ] && exit 0
/etc/init.d/sublink enable
exit 0
endef

define Package/sublink/prerm
#!/bin/sh
[ -n "$${IPKG_INSTROOT}" ] && exit 0
/etc/init.d/sublink disable 2>/dev/null
exit 0
endef

$(eval $(call BuildPackage,sublink))
