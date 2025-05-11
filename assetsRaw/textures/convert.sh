

# # config1:
# IMGID="03"
# POSITION="1024x820+1024+412"
# ASSET_NAME="screen_"
# GROUP="Panels"


# config2
IMGID="01"
POSITION="2028x1365+0+0"
ASSET_NAME="panel1_"
GROUP="FabricPanels"

convert Scifi_${GROUP}_${IMGID}_ambientocclusion.jpg -crop $POSITION ../../assets/textures/${ASSET_NAME}ambientocclusion.png
convert Scifi_${GROUP}_${IMGID}_basecolor.jpg -crop $POSITION ../../assets/textures/${ASSET_NAME}basecolor.png
#convert Scifi_${GROUP}_${IMGID}_emissive.jpg -crop $POSITION ../../assets/textures/${ASSET_NAME}emissive.png
convert Scifi_${GROUP}_${IMGID}_roughness.jpg -crop $POSITION ../../assets/textures/${ASSET_NAME}roughness.png
convert Scifi_${GROUP}_${IMGID}_normal.jpg -crop $POSITION ../../assets/textures/${ASSET_NAME}normal.png



# those are unused in the project:

# convert Scifi_${GROUP}_${IMGID}_emissiveMask.png -crop $POSITION ../../assets/textures/${ASSET_NAME}emissiveMask.png
# convert Scifi_${GROUP}_${IMGID}_height.png -crop $POSITION ../../assets/textures/${ASSET_NAME}height.png

